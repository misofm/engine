//! The ways rustc reads source that the walk does not: `#[path]` attributes and `include!`.
//!
//! The walk reads each `.rs` file under the roots and follows no module declaration, so a module
//! that `#[path = ".."]` points at, or code that `include!` pulls in, would reach rustc and no
//! rule. This scan runs over the tokens of every file and refuses each such form except two
//! that read nothing a rule needs to judge (#1519 D1):
//!
//! - a `#[path = "x.rs"]` that names a relative `.rs` file with no `..`, `target` or
//!   `node_modules` component, in an attribute that is neither inside `cfg_attr` arguments nor
//!   inside a macro token tree (a later rule resolves these declarations, and can resolve only
//!   a declaration it sees whole);
//! - an `include!("file")` of a file that holds literals and punctuation only, such as a digest
//!   array.
//!
//! `include_str!` and `include_bytes!` produce a literal and are not read outside a macro token
//! tree.
//!
//! Inside a macro token tree the scan refuses more (root Amendment 1 to #1519), because it cannot
//! follow what rustc builds there:
//!
//! - an identifier `include` in any form: an `include!` there is read relative to the file that
//!   expands the macro, which the scan cannot judge (rule 3);
//! - `include_str!` and `include_bytes!`, in any form (rule 4, measured to have no real site);
//!
//! and everywhere outside a `use` item, an identifier `include` that is not directly followed by
//! `!` is refused, because a macro can splice the `!` on (`inc!(include)`, rule 2).
//!
//! The matching of `name ! group` as a macro also matches `if !(..)`. That can only over-refuse
//! a condition that names `include`, `include_str` or `include_bytes`, and none exists.
//!
//! Rule 1 (root, 2026-10-12): inside a macro token tree, D1 (a) applies to written attributes. A
//! written `#[..]` or `#![..]` that holds an identifier `path` is refused there, whatever its
//! value. A `path` that is not inside a written attribute is not refused. A path attribute that
//! a macro expansion assembles is a stated limit of the tool (#1438 Hazards, limit 4).

use std::fs;
use std::path::Path;
use std::str::FromStr;

use proc_macro2::{Delimiter, Group, Ident, TokenStream, TokenTree};

use crate::report::{Class, Finding};
use crate::source::Source;

/// Where a token stream stands.
#[derive(Clone, Copy)]
struct Context {
    in_macro: bool,
    in_use: bool,
}

/// Scan `source`; `absolute` is the file's path on disk, whose directory a relative `include!`
/// argument is read from.
pub(crate) fn scan(source: &Source, absolute: &Path, findings: &mut Vec<Finding>) {
    let mut scan = Scan {
        directory: absolute.parent(),
        found: Vec::new(),
    };
    scan.stream(&source.tokens, false, false);
    scan.found.sort();
    scan.found.dedup();
    for (line, class) in scan.found {
        findings.push(Finding::at_line(
            class,
            &source.path,
            line,
            source.line_text(line),
        ));
    }
}

struct Scan<'a> {
    /// The directory of the scanned file, when it has one.
    directory: Option<&'a Path>,
    /// `(line, class)` of each refused form.
    found: Vec<(usize, Class)>,
}

impl Scan<'_> {
    /// Scan one token stream. `in_macro` is true inside a macro token tree: a `macro_rules!` or
    /// `macro` definition, or a macro invocation's delimited tokens. `in_use` is true inside a
    /// `use` item.
    fn stream(&mut self, stream: &TokenStream, in_macro: bool, in_use: bool) {
        let tokens: Vec<TokenTree> = stream.clone().into_iter().collect();
        // The groups of this stream that are macro token trees, and the tokens of `use` items,
        // by index.
        let mut macro_trees = vec![false; tokens.len()];
        let mut use_items = vec![in_use; tokens.len()];
        for (index, token) in tokens.iter().enumerate() {
            match token {
                TokenTree::Punct(punct) if punct.as_char() == '#' => {
                    let mut at = index + 1;
                    if matches!(tokens.get(at), Some(TokenTree::Punct(bang)) if bang.as_char() == '!')
                    {
                        at += 1;
                    }
                    if let Some(TokenTree::Group(group)) = tokens.get(at)
                        && group.delimiter() == Delimiter::Bracket
                    {
                        self.attribute(&group.stream(), in_macro);
                    }
                }
                TokenTree::Ident(ident) => {
                    self.ident(
                        &tokens,
                        index,
                        ident,
                        Context {
                            in_macro,
                            in_use: use_items[index],
                        },
                        (&mut macro_trees, &mut use_items),
                    );
                }
                _ => {}
            }
        }
        for (index, token) in tokens.iter().enumerate() {
            if let TokenTree::Group(group) = token {
                self.stream(
                    &group.stream(),
                    in_macro || macro_trees[index],
                    use_items[index],
                );
            }
        }
    }

    /// The forms that start at an identifier of `tokens`: a macro invocation or definition (whose
    /// groups are marked), `include!`, a `use` item (whose tokens are marked), and the words that
    /// are refused where they stand.
    fn ident(
        &mut self,
        tokens: &[TokenTree],
        index: usize,
        ident: &Ident,
        context: Context,
        (macro_trees, use_items): (&mut [bool], &mut [bool]),
    ) {
        let word = name(ident);
        let after = |offset: usize| tokens.get(index + offset);
        let bang_follows =
            matches!(after(1), Some(TokenTree::Punct(bang)) if bang.as_char() == '!');
        match (after(1), after(2)) {
            // `name!(..)`, `path::name!{..}`: the group is a macro invocation's token tree.
            (Some(TokenTree::Punct(bang)), Some(TokenTree::Group(group)))
                if bang.as_char() == '!' =>
            {
                macro_trees[index + 2] = true;
                if word == "include" && !context.in_macro && !context.in_use {
                    self.judge_include(group, ident);
                }
            }
            // `macro_rules! name { .. }`
            (Some(TokenTree::Punct(bang)), Some(TokenTree::Ident(_)))
                if bang.as_char() == '!' && word == "macro_rules" =>
            {
                if matches!(after(3), Some(TokenTree::Group(_))) {
                    macro_trees[index + 3] = true;
                }
            }
            // `macro name(..) { .. }` and `macro name { .. }`
            (Some(TokenTree::Ident(_)), Some(TokenTree::Group(first))) if word == "macro" => {
                macro_trees[index + 2] = true;
                if first.delimiter() == Delimiter::Parenthesis
                    && matches!(after(3), Some(TokenTree::Group(_)))
                {
                    macro_trees[index + 3] = true;
                }
            }
            _ => {}
        }
        if word == "use"
            && !matches!(after(1), Some(TokenTree::Punct(open)) if open.as_char() == '<')
        {
            // The item's tokens, up to its `;` (or the end of the stream).
            let rest = &tokens[index + 1..];
            let length = rest
                .iter()
                .position(|token| matches!(token, TokenTree::Punct(semi) if semi.as_char() == ';'))
                .unwrap_or(rest.len());
            use_items[index + 1..index + 1 + length].fill(true);
        }
        match word.as_str() {
            // An `include` in a `use` item renames the macro past the `include!` rule.
            "include" if context.in_use => self.refuse(ident, Class::RefusedIncludeImport),
            // In a macro token tree an `include` in any form is refused (an `include!` there is
            // read relative to the expanding file); anywhere else one without a `!` is.
            "include" if context.in_macro || !bang_follows => {
                self.refuse(ident, Class::RefusedInclude);
            }
            // `include_str!` and `include_bytes!` produce a literal outside a macro token tree
            // and are allowed there; inside one they are refused in any form.
            "include_str" | "include_bytes" if context.in_macro => {
                self.refuse(ident, Class::RefusedIncludeLiteralInMacro);
            }
            _ => {}
        }
    }

    /// An attribute's tokens (between its brackets).
    fn attribute(&mut self, attribute: &TokenStream, in_macro: bool) {
        let tokens: Vec<TokenTree> = attribute.clone().into_iter().collect();
        let in_cfg_attr =
            matches!(tokens.first(), Some(TokenTree::Ident(head)) if name(head) == "cfg_attr");
        self.attribute_tokens(&tokens, in_macro || in_cfg_attr);
    }

    /// Every `path` in `tokens`, at any depth. `refused` is true where no `path` is accepted.
    fn attribute_tokens(&mut self, tokens: &[TokenTree], refused: bool) {
        for (index, token) in tokens.iter().enumerate() {
            match token {
                TokenTree::Ident(ident) if name(ident) == "path" => {
                    if refused || !names_a_plain_file(tokens.get(index + 1), tokens.get(index + 2))
                    {
                        self.refuse(ident, Class::RefusedPathAttribute);
                    }
                }
                TokenTree::Group(group) => {
                    let inner: Vec<TokenTree> = group.stream().into_iter().collect();
                    self.attribute_tokens(&inner, refused);
                }
                _ => {}
            }
        }
    }

    /// `include!(<group>)` at `ident`, outside a macro token tree.
    fn judge_include(&mut self, group: &Group, ident: &Ident) {
        let accepted = string_literal(&group.stream())
            .and_then(|argument| {
                let path = Path::new(&argument);
                if path.is_relative() {
                    self.directory.map(|directory| directory.join(path))
                } else {
                    None
                }
            })
            .and_then(|file| fs::read_to_string(file).ok())
            .and_then(|text| TokenStream::from_str(&text).ok())
            .is_some_and(|tokens| literals_and_punctuation_only(&tokens));
        if !accepted {
            self.refuse(ident, Class::RefusedInclude);
        }
    }

    fn refuse(&mut self, ident: &Ident, class: Class) {
        let line = ident.span().start().line;
        self.found.push((line, class));
    }
}

/// An identifier's name without a raw-identifier prefix: rustc reads `r#path` as `path`.
fn name(ident: &Ident) -> String {
    let text = ident.to_string();
    text.strip_prefix("r#").map_or(text.clone(), str::to_owned)
}

/// True when `equals` and `value` are `=` and a plain string literal that names a file the tool
/// accepts as a `#[path]` (#1519 D1 (a)).
fn names_a_plain_file(equals: Option<&TokenTree>, value: Option<&TokenTree>) -> bool {
    let Some(TokenTree::Punct(equals)) = equals else {
        return false;
    };
    if equals.as_char() != '=' {
        return false;
    }
    let Some(TokenTree::Literal(literal)) = value else {
        return false;
    };
    let Ok(string) =
        syn::parse2::<syn::LitStr>(TokenStream::from(TokenTree::Literal(literal.clone())))
    else {
        return false;
    };
    let value = string.value();
    let mut components = value.split(['/', '\\']);
    value.ends_with(".rs")
        && !value.starts_with(['/', '\\'])
        && components.all(|component| !matches!(component, ".." | "target" | "node_modules"))
}

/// The value of `stream` when it is exactly one string literal.
fn string_literal(stream: &TokenStream) -> Option<String> {
    syn::parse2::<syn::LitStr>(stream.clone())
        .ok()
        .map(|string| string.value())
}

/// True when `stream` holds literals, punctuation and groups of them, and no identifier.
fn literals_and_punctuation_only(stream: &TokenStream) -> bool {
    stream.clone().into_iter().all(|token| match token {
        TokenTree::Group(group) => literals_and_punctuation_only(&group.stream()),
        TokenTree::Literal(_) | TokenTree::Punct(_) => true,
        TokenTree::Ident(_) => false,
    })
}
