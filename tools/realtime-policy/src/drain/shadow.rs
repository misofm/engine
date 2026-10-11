//! Whether a file redefines or imports a macro named `debug_assert` (B2a-D2).
//!
//! The drain rule takes `debug_assert!` at its word: its arguments are parsed as expressions, so
//! a count named in one is a read and not a rebinding (B1a-D7). A macro of the same name, defined
//! or imported anywhere under the scan roots, can reach any file through `#[macro_use] mod` or a
//! glob `use`, so one file that defines or imports it withdraws that trust from every file.
//!
//! The scan reads the file's tokens, at any depth, and so sees a `macro_rules!` that sits in a
//! function body or in another macro's token tree. It reports:
//!
//! - `macro_rules! debug_assert` and `macro debug_assert`;
//! - a `use` item that holds the identifier `debug_assert` anywhere: `use x::debug_assert;`,
//!   `use x::y as debug_assert;`, `use x::{debug_assert, z};`. A path through a module of that
//!   name is reported too; refusing it is harmless.
//!
//! Stated limit: a registry crate that exports a `debug_assert` macro and is imported with
//! `#[macro_use] extern crate` or a glob `use` is outside the check. No crate in the workspace
//! does this.

use proc_macro2::{TokenStream, TokenTree};

use super::names::{bare, tokens_name};

const NAME: &str = "debug_assert";

/// Whether `tokens` define or import a macro named `debug_assert`.
pub(crate) fn debug_assert_shadowed(tokens: &TokenStream) -> bool {
    let trees: Vec<TokenTree> = tokens.clone().into_iter().collect();
    (0..trees.len()).any(|at| match &trees[at] {
        TokenTree::Ident(word) => match bare(word).as_str() {
            "macro_rules" => is_bang(trees.get(at + 1)) && is_name(trees.get(at + 2)),
            "macro" => is_name(trees.get(at + 1)),
            "use" => use_item_names(&trees[at + 1..]),
            _ => false,
        },
        TokenTree::Group(group) => debug_assert_shadowed(&group.stream()),
        _ => false,
    })
}

fn is_bang(tree: Option<&TokenTree>) -> bool {
    matches!(tree, Some(TokenTree::Punct(punct)) if punct.as_char() == '!')
}

fn is_name(tree: Option<&TokenTree>) -> bool {
    matches!(tree, Some(TokenTree::Ident(word)) if bare(word) == NAME)
}

/// Whether the tokens after a `use` keyword, up to the `;` that ends the item, hold the name. A
/// `use<..>` precise-capturing bound is not an item.
fn use_item_names(after: &[TokenTree]) -> bool {
    if matches!(after.first(), Some(TokenTree::Punct(punct)) if punct.as_char() == '<') {
        return false;
    }
    after
        .iter()
        .take_while(|tree| !matches!(tree, TokenTree::Punct(punct) if punct.as_char() == ';'))
        .any(|tree| match tree {
            TokenTree::Ident(word) => bare(word) == NAME,
            TokenTree::Group(group) => tokens_name(&group.stream(), NAME),
            _ => false,
        })
}
