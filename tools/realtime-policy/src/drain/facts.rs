//! What one visit of a parsed file records: its pops, its uses of local names, and its parsed
//! `debug_assert!` arguments.

use std::collections::HashMap;

use syn::parse::Parser;
use syn::visit::{self, Visit};

use super::names::bare;
use super::tree::{Pos, is_debug_assert, macro_key, pos};

/// The one name that a pop spells (D5).
pub(super) const POP: &str = "try_pop";

/// How a pop is spelled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PopKind {
    /// A method call named `try_pop`.
    Method,
    /// A path expression whose last segment is `try_pop`.
    Path,
    /// The identifier `try_pop` inside a macro's token tree.
    MacroToken,
}

/// A pop and the position of its `try_pop` identifier.
#[derive(Clone, Copy, Debug)]
pub(super) struct PopSite {
    pub(super) pos: Pos,
    pub(super) kind: PopKind,
}

/// Every pop of `file`, in no particular order.
pub(super) fn pops(file: &syn::File) -> Vec<PopSite> {
    let mut finder = PopFinder { found: Vec::new() };
    finder.visit_file(file);
    finder.found
}

struct PopFinder {
    found: Vec<PopSite>,
}

impl PopFinder {
    fn tokens(&mut self, tokens: &proc_macro2::TokenStream) {
        for token in tokens.clone() {
            match token {
                proc_macro2::TokenTree::Ident(ident) if bare(&ident) == POP => {
                    self.found.push(PopSite {
                        pos: pos(ident.span().start()),
                        kind: PopKind::MacroToken,
                    });
                }
                proc_macro2::TokenTree::Group(group) => self.tokens(&group.stream()),
                _ => {}
            }
        }
    }
}

impl<'ast> Visit<'ast> for PopFinder {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if bare(&node.method) == POP {
            self.found.push(PopSite {
                pos: pos(node.method.span().start()),
                kind: PopKind::Method,
            });
        }
        visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        if let Some(last) = node.path.segments.last()
            && bare(&last.ident) == POP
        {
            self.found.push(PopSite {
                pos: pos(last.ident.span().start()),
                kind: PopKind::Path,
            });
        }
        visit::visit_expr_path(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        self.tokens(&node.tokens);
    }
}

/// How a name is used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum UseKind {
    /// A single-segment path expression.
    Path,
    /// An identifier in a macro's token tree, where no syntax is parsed.
    Macro,
}

/// A place where a name could be read or changed.
#[derive(Clone, Copy, Debug)]
pub(super) struct Use {
    pub(super) pos: Pos,
    pub(super) kind: UseKind,
}

/// The uses of every name and the parsed `debug_assert!` arguments of one file.
#[derive(Default)]
pub(super) struct Facts {
    /// The uses of each name (a raw identifier is its bare name), sorted by position.
    pub(super) uses: HashMap<String, Vec<Use>>,
    /// The arguments of each `debug_assert!`, parsed as expressions, by [`macro_key`].
    pub(super) debug_args: HashMap<Pos, Vec<syn::Expr>>,
}

impl Facts {
    pub(super) fn of(file: &syn::File) -> Self {
        let mut facts = Self::default();
        facts.visit_file(file);
        for uses in facts.uses.values_mut() {
            uses.sort_by_key(|used| used.pos);
        }
        facts
    }

    /// The uses of `name` that start in `from..to`.
    pub(super) fn uses_in(&self, name: &str, from: Pos, to: Pos) -> &[Use] {
        let Some(uses) = self.uses.get(name) else {
            return &[];
        };
        let first = uses.partition_point(|used| used.pos < from);
        let last = uses.partition_point(|used| used.pos < to);
        &uses[first..last.max(first)]
    }

    fn record(&mut self, name: String, at: Pos, kind: UseKind) {
        self.uses
            .entry(name)
            .or_default()
            .push(Use { pos: at, kind });
    }

    fn tokens(&mut self, tokens: &proc_macro2::TokenStream) {
        for token in tokens.clone() {
            match token {
                proc_macro2::TokenTree::Ident(ident) => {
                    self.record(bare(&ident), pos(ident.span().start()), UseKind::Macro);
                }
                proc_macro2::TokenTree::Group(group) => self.tokens(&group.stream()),
                _ => {}
            }
        }
    }
}

impl<'ast> Visit<'ast> for Facts {
    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        if node.qself.is_none()
            && node.path.leading_colon.is_none()
            && node.path.segments.len() == 1
            && let Some(segment) = node.path.segments.first()
            && segment.arguments.is_none()
        {
            self.record(
                bare(&segment.ident),
                pos(segment.ident.span().start()),
                UseKind::Path,
            );
        }
        visit::visit_expr_path(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        let parsed = is_debug_assert(node)
            .then(|| {
                syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated
                    .parse2(node.tokens.clone())
                    .ok()
            })
            .flatten();
        let Some(args) = parsed else {
            self.tokens(&node.tokens);
            return;
        };
        let args: Vec<syn::Expr> = args.into_iter().collect();
        let mut inner = Self::default();
        for arg in &args {
            inner.visit_expr(arg);
        }
        for (name, uses) in inner.uses {
            self.uses.entry(name).or_default().extend(uses);
        }
        self.debug_args.extend(inner.debug_args);
        self.debug_args.insert(macro_key(node), args);
    }
}
