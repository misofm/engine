//! The trusted-name scan (B2a-D2 F2, root Amendment 4): the names that the drain rule takes at their
//! word, and every place under the scan roots that could give one of them another meaning.
//!
//! The drain rule trusts three things. `debug_assert!` is the standard macro, so a count named in
//! its arguments is a read and not a rebinding (B1a-D7). `try_pop` is the one pop, and
//! `available_at_entry` is the one count (`Q` below). A definition, an import, a rename or a
//! metaprogramming escape for one of them can reach any file (`#[macro_use] mod`, a glob `use`, a
//! trait that is in scope, the implicit prelude), so the scan reads the tokens of every `.rs` file
//! under the roots, at any depth, and refuses:
//!
//! - **(q)** A `Q` identifier (`try_pop`, `available_at_entry`; bare or `r#`) that does not directly
//!   follow `.` or `::`, except exactly one written `fn` token for each of the two, in
//!   `crates/engine/src/realtime/spsc.rs`. A second definition in that file is refused, and so is
//!   every other written definition of either name (a `fn`, binding, field, label, macro, braced
//!   import or `use .. as` rename). An import by a `::` path, such as `pub use crate::loops::try_pop;`, is not
//!   refused: it can only name a written definition that this rule refuses, a registry item
//!   (limit 1) or an expansion (limit 4).
//! - **(s)** A `debug_assert` identifier (bare or `r#`) that a `!` does not directly follow (F2). It
//!   is a shadow: no file's `debug_assert!` is then taken at its word, and a count named in one is
//!   refused everywhere, as in any other macro. One rule covers `macro_rules! debug_assert`,
//!   `macro debug_assert`, `use .. as debug_assert`, `use x::debug_assert` and a proc macro's
//!   `fn debug_assert`. It is not generalized to other assert macros: nothing else consumes that
//!   trust. Separately, only a single-identifier `debug_assert!` is taken at its word (a path,
//!   `math::debug_assert!`, can be any macro).
//! - **(u)** A `macro_use` identifier, except the whole attribute `#[macro_use]` directly followed by
//!   `mod`. An attribute on `extern crate` brings a registry crate's macros into every module, and
//!   `#[macro_use] mod` is the only form the workspace uses.
//! - **(n)** Any `no_implicit_prelude` identifier. It changes what the implicit prelude (and so
//!   `debug_assert!`) names in its module.
//! - **(d)** Any identifier `proc_macro`, `proc_macro_attribute` or `proc_macro_derive`. A workspace
//!   proc macro can emit any name, pop or shadow, and the scan cannot see it.
//! - **(f)** An identifier `feature` directly followed by a `(` group, and a `$` directly followed by
//!   a `{` group (a metavariable expression, so that a stabilized `${concat(..)}` is still refused
//!   after a toolchain bump). `#![feature(..)]` and `#![cfg_attr(x, feature(..))]` enable the
//!   nightly features that build names from parts.
//!
//! A hit is a finding of the drain class at the line of the identifier. Names and paths that a
//! macro expansion builds from parts (a `fn $n` made by a `macro_rules!` fragment, a `tt` that
//! carries `fn`, a macro made by a macro) are limit 4 of the threat model (#1438 Hazards, owner
//! decision of 2026-10-11): the scan judges every written token and does not model an expansion. A
//! registry crate's macros and proc macros, and the standard library, are limit 1 (code outside the
//! workspace).

use std::collections::BTreeSet;

use proc_macro2::{Delimiter, Span, TokenTree};

use super::names::bare;
use crate::report::{Class, Finding};
use crate::source::Source;

/// The names that only the queue's own definition may spell: the pop and the count.
const Q: [&str; 2] = ["try_pop", "available_at_entry"];

/// The one file that defines them.
const DEFINING_FILE: &str = "crates/engine/src/realtime/spsc.rs";

/// The identifiers of (d).
const PROC_MACRO: [&str; 3] = ["proc_macro", "proc_macro_attribute", "proc_macro_derive"];

/// What the scan has read of the files so far.
#[derive(Default)]
pub(crate) struct NameScan {
    /// F2: a `debug_assert` that no `!` follows was found.
    shadowed: bool,
    /// The hits of (q), (u), (n), (d) and (f).
    findings: Vec<Finding>,
    /// The names of `Q` that `DEFINING_FILE` has defined so far: one written `fn` each.
    defined: BTreeSet<&'static str>,
}

impl NameScan {
    /// Read one file.
    pub(crate) fn read(&mut self, source: &Source) {
        let trees: Vec<TokenTree> = source.tokens.clone().into_iter().collect();
        self.walk(source, &trees);
    }

    /// The end of the walk: whether no file shadows `debug_assert` (F2), and the findings, sorted.
    pub(crate) fn finish(mut self) -> (bool, Vec<Finding>) {
        self.findings.sort();
        self.findings.dedup();
        (!self.shadowed, self.findings)
    }

    fn walk(&mut self, source: &Source, trees: &[TokenTree]) {
        for (at, tree) in trees.iter().enumerate() {
            match tree {
                TokenTree::Group(group) => {
                    if !is_macro_use_on_a_module(trees, at) {
                        let inner: Vec<TokenTree> = group.stream().into_iter().collect();
                        self.walk(source, &inner);
                    }
                }
                TokenTree::Ident(word) => self.word(source, trees, at, &bare(word), word.span()),
                TokenTree::Punct(punct)
                    if punct.as_char() == '$' && is_group(trees.get(at + 1), Delimiter::Brace) =>
                {
                    self.findings.push(hit(source, punct.span()));
                }
                _ => {}
            }
        }
    }

    fn word(&mut self, source: &Source, trees: &[TokenTree], at: usize, name: &str, span: Span) {
        if let Some(trusted) = Q.iter().find(|trusted| **trusted == name) {
            if follows_path(trees, at) {
                return;
            }
            let written_fn = at.checked_sub(1).is_some_and(
                |before| matches!(&trees[before], TokenTree::Ident(word) if word == "fn"),
            );
            // The first written `fn` of each name in the defining file is the one definition.
            if !(written_fn && source.path == DEFINING_FILE && self.defined.insert(trusted)) {
                self.findings.push(hit(source, span));
            }
            return;
        }
        match name {
            "debug_assert" if !is_punct(trees.get(at + 1), '!') => self.shadowed = true,
            "macro_use" | "no_implicit_prelude" => self.findings.push(hit(source, span)),
            "feature" if is_group(trees.get(at + 1), Delimiter::Parenthesis) => {
                self.findings.push(hit(source, span));
            }
            other if PROC_MACRO.contains(&other) => self.findings.push(hit(source, span)),
            _ => {}
        }
    }
}

fn hit(source: &Source, span: Span) -> Finding {
    let line = span.start().line;
    Finding::at_line(
        Class::UnboundedTryPop,
        &source.path,
        line,
        source.line_text(line),
    )
}

fn is_punct(tree: Option<&TokenTree>, character: char) -> bool {
    matches!(tree, Some(TokenTree::Punct(punct)) if punct.as_char() == character)
}

fn is_group(tree: Option<&TokenTree>, delimiter: Delimiter) -> bool {
    matches!(tree, Some(TokenTree::Group(group)) if group.delimiter() == delimiter)
}

/// Whether the identifier at `trees[at]` directly follows a single `.` or `::`.
fn follows_path(trees: &[TokenTree], at: usize) -> bool {
    let before = |back: usize| at.checked_sub(back).and_then(|index| trees.get(index));
    match before(1) {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '.' => !is_punct(before(2), '.'),
        Some(TokenTree::Punct(punct)) if punct.as_char() == ':' => is_punct(before(2), ':'),
        _ => false,
    }
}

/// Whether `trees[at]` is the bracket group of the whole attribute `#[macro_use]` (an outer
/// attribute: `#` directly before it, no `!`) and `mod` directly follows it. (u) leaves that one
/// attribute alone and reads every other group.
fn is_macro_use_on_a_module(trees: &[TokenTree], at: usize) -> bool {
    let Some(TokenTree::Group(group)) = trees.get(at) else {
        return false;
    };
    if group.delimiter() != Delimiter::Bracket
        || !at
            .checked_sub(1)
            .is_some_and(|before| is_punct(trees.get(before), '#'))
        || !matches!(trees.get(at + 1), Some(TokenTree::Ident(word)) if word == "mod")
    {
        return false;
    }
    let inner: Vec<TokenTree> = group.stream().into_iter().collect();
    matches!(inner.as_slice(), [TokenTree::Ident(word)] if word == "macro_use")
}
