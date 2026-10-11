//! The trusted-name scan (B2a-D2 F2, B2a-D3 SP-A): the names that the drain rule takes at their
//! word, and every place under the scan roots that could give one of them another meaning.
//!
//! The drain rule trusts four names. `debug_assert!` is the standard macro, so a count named in
//! its arguments is a read and not a rebinding (B1a-D7). `.map(<path>)` on `<fields>.as_mut()` is
//! `Option::map`, which runs its argument at most once (B2a-D3). `try_pop` is the one pop. A
//! definition, an import, a rename or a macro-emitted use of one of these names can reach any file
//! (`#[macro_use] mod`, a glob `use`, a trait that is in scope), so the scan reads the tokens of
//! every `.rs` file under the roots, at any depth, and refuses:
//!
//! - **F2.** A `debug_assert` identifier (bare or `r#`) that a `!` does not directly follow. It is
//!   a shadow: no file's `debug_assert!` is then taken at its word, and a count named in one is
//!   refused everywhere, as in any other macro. One rule covers `macro_rules! debug_assert`,
//!   `macro debug_assert`, `use .. as debug_assert`, `use x::debug_assert` and a proc macro's
//!   `fn debug_assert`.
//! - **(a)** An identifier `map` or `as_mut` that does not directly follow `.` or `::`, where it
//!   defines or imports a name: after `fn`, `const`, `static`, `struct`, `enum`, `union`, `type`,
//!   `trait`, `mod`, `macro`, `as` or `macro_rules!`, or anywhere in a `use` item. (Root's text
//!   refuses every such identifier, a local variable included. The workspace holds 17 local
//!   variables named `map`, in `tools/console-workload/tests/paired_spans.rs` and
//!   `tools/session-validator/src/lib.rs`, so the text cannot pass the real tree as worded; a
//!   local variable cannot change what `.map(..)` resolves to. The attempt record of #1442 names
//!   this deviation for root's ruling.)
//! - **(b)** An identifier `try_pop` that does not directly follow `.` or `::`, except in
//!   `fn try_pop(&mut self`, with the receiver tokens exactly `&`, `mut`, `self`.
//! - **(c)** An identifier `map`, `as_mut`, `try_pop` or `debug_assert` in the token tree of an
//!   invocation of a name-emitting macro. A name-emitting macro is a `macro_rules!` or `macro`
//!   under the roots whose body holds `fn` directly followed by `$`, or that invokes a
//!   name-emitting macro (the set is closed transitively). An invocation is matched by its last
//!   path segment, and an invocation through a name that a `use` item renames (`as n`) counts.
//! - **(d)** Any identifier `proc_macro`, `proc_macro_attribute` or `proc_macro_derive`. A
//!   workspace proc macro can emit any name, pop or shadow, and the scan cannot see it.
//!
//! A hit of (a) to (d) is a finding of the drain class at the line of the identifier. A registry
//! trait or type method named `map` or `as_mut`, a registry crate's macros and proc macros, and
//! the standard library are outside the scan: the threat-model limits of #1438.

use std::collections::BTreeSet;

use proc_macro2::{Delimiter, Spacing, TokenTree};

use super::names::bare;
use crate::report::{Class, Finding};
use crate::source::Source;

/// The names that (c) looks for in a name-emitting macro's invocation.
const TRUSTED: [&str; 4] = ["map", "as_mut", "try_pop", "debug_assert"];

/// The words after which an identifier is the name of an item, a macro or a rename.
const ITEM_KEYWORDS: [&str; 11] = [
    "fn", "const", "static", "struct", "enum", "union", "type", "trait", "mod", "macro", "as",
];

/// The identifiers of (d).
const PROC_MACRO: [&str; 3] = ["proc_macro", "proc_macro_attribute", "proc_macro_derive"];

/// What the scan has read of the files so far.
#[derive(Default)]
pub(crate) struct NameScan {
    /// F2: a `debug_assert` that no `!` follows was found.
    shadowed: bool,
    /// The hits of (a), (b) and (d).
    findings: Vec<Finding>,
    /// Every `macro_rules!` and `macro` definition.
    macros: Vec<MacroDef>,
    /// Every `x as y` of a `use` item, as (`y`, `x`).
    renames: Vec<(String, String)>,
    /// Every invocation whose token tree holds a trusted name, with that name's hits.
    invocations: Vec<Invocation>,
}

struct MacroDef {
    name: String,
    /// The body holds `fn` directly followed by `$`.
    emits_names: bool,
    /// The names of the macros that the body invokes.
    invokes: Vec<String>,
}

struct Invocation {
    name: String,
    hits: Vec<Finding>,
}

impl NameScan {
    /// Read one file.
    pub(crate) fn read(&mut self, source: &Source) {
        let trees: Vec<TokenTree> = source.tokens.clone().into_iter().collect();
        self.walk(source, &trees);
    }

    /// The end of the walk: whether no file shadows `debug_assert` (F2), and the findings of
    /// (a) to (d), sorted.
    pub(crate) fn finish(mut self) -> (bool, Vec<Finding>) {
        let mut emitting: BTreeSet<String> = self
            .macros
            .iter()
            .filter(|definition| definition.emits_names)
            .map(|definition| definition.name.clone())
            .collect();
        // The set is closed: a macro that invokes a name-emitting macro emits names too, and so
        // does a name that renames one.
        let mut changed = true;
        while changed {
            changed = false;
            for definition in &self.macros {
                if !emitting.contains(&definition.name)
                    && definition
                        .invokes
                        .iter()
                        .any(|name| emitting.contains(name))
                {
                    emitting.insert(definition.name.clone());
                    changed = true;
                }
            }
            for (alias, original) in &self.renames {
                if emitting.contains(original) && emitting.insert(alias.clone()) {
                    changed = true;
                }
            }
        }
        for invocation in self.invocations {
            if emitting.contains(&invocation.name) {
                self.findings.extend(invocation.hits);
            }
        }
        self.findings.sort();
        self.findings.dedup();
        (!self.shadowed, self.findings)
    }

    fn walk(&mut self, source: &Source, trees: &[TokenTree]) {
        for (at, tree) in trees.iter().enumerate() {
            match tree {
                TokenTree::Group(group) => {
                    let inner: Vec<TokenTree> = group.stream().into_iter().collect();
                    self.walk(source, &inner);
                }
                TokenTree::Ident(word) => self.word(source, trees, at, &bare(word), word),
                _ => {}
            }
        }
    }

    fn word(
        &mut self,
        source: &Source,
        trees: &[TokenTree],
        at: usize,
        name: &str,
        word: &proc_macro2::Ident,
    ) {
        let hit = || hit(source, word);
        match name {
            "map" | "as_mut" if !follows_path(trees, at) && defines_a_name(trees, at) => {
                self.findings.push(hit());
            }
            "try_pop" if !follows_path(trees, at) && !pop_definition(trees, at) => {
                self.findings.push(hit());
            }
            "debug_assert" if !is_bang(trees.get(at + 1)) => self.shadowed = true,
            other if PROC_MACRO.contains(&other) => self.findings.push(hit()),
            "macro_rules" if is_bang(trees.get(at + 1)) => self.define(trees, at + 2),
            "macro" => self.define(trees, at + 1),
            "use" => self.import(source, &trees[at + 1..]),
            _ => {}
        }
        if is_bang(trees.get(at + 1))
            && let Some(TokenTree::Group(group)) = trees.get(at + 2)
        {
            let mut hits = Vec::new();
            trusted_names(source, group, &mut hits);
            if !hits.is_empty() {
                self.invocations.push(Invocation {
                    name: name.to_owned(),
                    hits,
                });
            }
        }
    }

    /// Record the definition whose name is `trees[at]` and whose body groups follow.
    fn define(&mut self, trees: &[TokenTree], at: usize) {
        let Some(TokenTree::Ident(name)) = trees.get(at) else {
            return;
        };
        let mut definition = MacroDef {
            name: bare(name),
            emits_names: false,
            invokes: Vec::new(),
        };
        // `macro_rules! m { .. }` has one group, and `macro m($x:ident) { .. }` has an argument
        // group and then the brace group of the body: read groups up to the first brace group.
        for tree in &trees[at + 1..] {
            let TokenTree::Group(group) = tree else {
                break;
            };
            let inner: Vec<TokenTree> = group.stream().into_iter().collect();
            definition.emits_names |= holds_fn_dollar(&inner);
            invoked(&inner, &mut definition.invokes);
            if group.delimiter() == Delimiter::Brace {
                break;
            }
        }
        self.macros.push(definition);
    }

    /// Read the `use` item whose tokens follow its keyword: its renames (`x as y`), and a `map` or
    /// `as_mut` that it brings into scope under a name of its own (`use x::{map, y};`).
    fn import(&mut self, source: &Source, after: &[TokenTree]) {
        let item: Vec<TokenTree> = after
            .iter()
            .take_while(|tree| !matches!(tree, TokenTree::Punct(punct) if punct.as_char() == ';'))
            .cloned()
            .collect();
        renames(&item, &mut self.renames);
        imported_names(source, &item, &mut self.findings);
    }
}

fn hit(source: &Source, word: &proc_macro2::Ident) -> Finding {
    let line = word.span().start().line;
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

fn is_bang(tree: Option<&TokenTree>) -> bool {
    is_punct(tree, '!')
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

/// Whether the identifier at `trees[at]` stands where an item takes its name: after `fn`, `const`,
/// `static`, `struct`, `enum`, `union`, `type`, `trait`, `mod`, `macro`, `as` or `macro_rules!`.
fn defines_a_name(trees: &[TokenTree], at: usize) -> bool {
    let before = |back: usize| at.checked_sub(back).and_then(|index| trees.get(index));
    match before(1) {
        Some(TokenTree::Ident(word)) => ITEM_KEYWORDS.iter().any(|keyword| word == keyword),
        Some(TokenTree::Punct(punct)) if punct.as_char() == '!' => {
            matches!(before(2), Some(TokenTree::Ident(word)) if word == "macro_rules")
        }
        _ => false,
    }
}

/// Whether `trees[at]`, a `try_pop`, is the name in `fn try_pop(&mut self`.
fn pop_definition(trees: &[TokenTree], at: usize) -> bool {
    let keyword = at
        .checked_sub(1)
        .is_some_and(|before| matches!(&trees[before], TokenTree::Ident(word) if word == "fn"));
    let Some(TokenTree::Group(parameters)) = trees.get(at + 1) else {
        return false;
    };
    if !keyword || parameters.delimiter() != Delimiter::Parenthesis {
        return false;
    }
    let tokens: Vec<TokenTree> = parameters.stream().into_iter().take(3).collect();
    matches!(
        tokens.as_slice(),
        [TokenTree::Punct(and), TokenTree::Ident(mutable), TokenTree::Ident(this)]
            if and.as_char() == '&' && and.spacing() == Spacing::Alone && mutable == "mut" && this == "self"
    )
}

/// Whether the tokens hold `fn` directly followed by `$`, at any depth.
fn holds_fn_dollar(trees: &[TokenTree]) -> bool {
    trees.iter().enumerate().any(|(at, tree)| match tree {
        TokenTree::Ident(word) => word == "fn" && is_punct(trees.get(at + 1), '$'),
        TokenTree::Group(group) => holds_fn_dollar(&group.stream().into_iter().collect::<Vec<_>>()),
        _ => false,
    })
}

/// The names of the macros invoked in the tokens, at any depth.
fn invoked(trees: &[TokenTree], names: &mut Vec<String>) {
    for (at, tree) in trees.iter().enumerate() {
        match tree {
            TokenTree::Ident(word)
                if is_bang(trees.get(at + 1))
                    && matches!(trees.get(at + 2), Some(TokenTree::Group(_))) =>
            {
                names.push(bare(word));
            }
            TokenTree::Group(group) => {
                invoked(&group.stream().into_iter().collect::<Vec<_>>(), names);
            }
            _ => {}
        }
    }
}

/// The trusted names in an invocation's token tree, at any depth.
fn trusted_names(source: &Source, group: &proc_macro2::Group, hits: &mut Vec<Finding>) {
    for tree in group.stream() {
        match tree {
            TokenTree::Ident(word) if TRUSTED.contains(&bare(&word).as_str()) => {
                hits.push(hit(source, &word));
            }
            TokenTree::Group(inner) => trusted_names(source, &inner, hits),
            _ => {}
        }
    }
}

/// The `map` and `as_mut` identifiers of a `use` item, at any depth, that do not follow `.` or `::`.
fn imported_names(source: &Source, trees: &[TokenTree], out: &mut Vec<Finding>) {
    for (at, tree) in trees.iter().enumerate() {
        match tree {
            TokenTree::Ident(word)
                if matches!(bare(word).as_str(), "map" | "as_mut") && !follows_path(trees, at) =>
            {
                out.push(hit(source, word));
            }
            TokenTree::Group(group) => {
                imported_names(source, &group.stream().into_iter().collect::<Vec<_>>(), out);
            }
            _ => {}
        }
    }
}

/// The `x as y` pairs in the tokens of a `use` item, at any depth, as (`y`, `x`).
fn renames(trees: &[TokenTree], out: &mut Vec<(String, String)>) {
    for (at, tree) in trees.iter().enumerate() {
        match tree {
            TokenTree::Ident(word) if word == "as" => {
                if let (Some(TokenTree::Ident(original)), Some(TokenTree::Ident(alias))) = (
                    at.checked_sub(1).and_then(|before| trees.get(before)),
                    trees.get(at + 1),
                ) {
                    out.push((bare(alias), bare(original)));
                }
            }
            TokenTree::Group(group) => {
                renames(&group.stream().into_iter().collect::<Vec<_>>(), out);
            }
            _ => {}
        }
    }
}
