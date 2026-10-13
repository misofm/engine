//! The whole-plan rule: the C ABI never forms a reference to a whole `Plan`.
//!
//! The render thread holds `&mut PlanState` while any thread may query the same handle, so the
//! production code of `crates/capi/src/ffi.rs` projects the fields it needs with `&raw const` or
//! `&raw mut` and never borrows `*plan`. In the syntax tree the rule is a `&` or `&mut` whose
//! operand, after parentheses, is `*plan`. A macro's token tree is not parsed, so there the rule
//! reads the same shape on tokens.

use std::collections::BTreeSet;

use syn::visit::{self, Visit};

use crate::leaf::{self, Kind, Leaf};
use crate::report::{Class, Finding};
use crate::source::Source;

/// The file the rule reads, relative to the workspace root.
pub(crate) const CAPI_FFI: &str = "crates/capi/src/ffi.rs";

/// The render entry point, which must be among the scanned items so that a renamed function or a
/// moved test module cannot pass as clean.
const RENDER_ENTRY: &str = "miso_engine_v1_render_f32_planar";

/// The name of the plan pointer in the C ABI's functions.
const PLAN: &str = "plan";

/// Check the production items of `file`, which is `CAPI_FFI` parsed from `source`.
pub(crate) fn scan(source: &Source, file: &syn::File, findings: &mut Vec<Finding>) {
    let mut scan = Scan::default();
    for item in file.items.iter().filter(|item| !is_test_module(item)) {
        scan.visit_item(item);
    }
    if !scan.has_render_entry {
        findings.push(Finding::at_path(Class::NoRenderEntry, &source.path));
    }
    findings.extend(scan.lines.into_iter().map(|line| {
        Finding::at_line(
            Class::WholePlanReference,
            &source.path,
            line,
            source.line_text(line),
        )
    }));
}

/// A top-level `mod tests` whose attributes include exactly `#[cfg(test)]`. A module under
/// `#[cfg(any(test, ..))]` is production code.
fn is_test_module(item: &syn::Item) -> bool {
    let syn::Item::Mod(module) = item else {
        return false;
    };
    module.ident == "tests"
        && module.attrs.iter().any(|attribute| {
            matches!(attribute.style, syn::AttrStyle::Outer)
                && attribute.path().is_ident("cfg")
                && attribute
                    .parse_args::<syn::Ident>()
                    .is_ok_and(|argument| argument == "test")
        })
}

#[derive(Default)]
struct Scan {
    /// The lines that hold a reference to the whole plan.
    lines: BTreeSet<usize>,
    /// Whether a production `fn` has the render entry point's name.
    has_render_entry: bool,
}

impl<'ast> Visit<'ast> for Scan {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.has_render_entry |= node.sig.ident == RENDER_ENTRY;
        visit::visit_item_fn(self, node);
    }

    fn visit_expr_reference(&mut self, node: &'ast syn::ExprReference) {
        if is_plan_deref(&node.expr) {
            self.lines.insert(node.and_token.span.start().line);
        }
        visit::visit_expr_reference(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        let leaves = leaf::flatten(&node.tokens);
        self.lines.extend(
            (0..leaves.len())
                .filter(|&index| borrows_whole_plan(&leaves, index))
                .map(|index| leaves[index].start.line),
        );
        visit::visit_macro(self, node);
    }
}

/// The expression without the parentheses around it.
fn unparenthesized(mut expr: &syn::Expr) -> &syn::Expr {
    while let syn::Expr::Paren(inner) = expr {
        expr = &inner.expr;
    }
    expr
}

/// Whether `operand` is `*plan`, with any parentheses around it or around `plan`.
fn is_plan_deref(operand: &syn::Expr) -> bool {
    let syn::Expr::Unary(unary) = unparenthesized(operand) else {
        return false;
    };
    matches!(unary.op, syn::UnOp::Deref(_))
        && matches!(
            unparenthesized(&unary.expr),
            syn::Expr::Path(path) if path.qself.is_none() && path.path.is_ident(PLAN)
        )
}

/// Whether the tokens from `at` are `&`, an optional `mut`, then `*plan` or `(*plan)`, and the next
/// token is absent or does not continue the operand (`.`, `(`, `[` or `::`).
fn borrows_whole_plan(leaves: &[Leaf], at: usize) -> bool {
    if !is_punct(leaves.get(at), '&') {
        return false;
    }
    let mut next = at + 1;
    if is_word(leaves.get(next), "mut") {
        next += 1;
    }
    let operand_end = if is_punct(leaves.get(next), '*') && is_word(leaves.get(next + 1), PLAN) {
        next + 2
    } else if matches!(
        leaves.get(next).map(|leaf| &leaf.kind),
        Some(Kind::Open('('))
    ) && is_punct(leaves.get(next + 1), '*')
        && is_word(leaves.get(next + 2), PLAN)
        && matches!(
            leaves.get(next + 3).map(|leaf| &leaf.kind),
            Some(Kind::Close(')'))
        )
    {
        next + 4
    } else {
        return false;
    };
    let continues = match leaves.get(operand_end).map(|leaf| &leaf.kind) {
        Some(Kind::Punct('.') | Kind::Open('(' | '[')) => true,
        Some(Kind::Punct(':')) => is_punct(leaves.get(operand_end + 1), ':'),
        _ => false,
    };
    !continues
}

fn is_punct(leaf: Option<&Leaf>, character: char) -> bool {
    matches!(leaf.map(|leaf| &leaf.kind), Some(Kind::Punct(found)) if *found == character)
}

fn is_word(leaf: Option<&Leaf>, word: &str) -> bool {
    matches!(leaf.map(|leaf| &leaf.kind), Some(Kind::Word(found)) if found == word)
}
