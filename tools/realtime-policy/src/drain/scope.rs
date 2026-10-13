//! The scope resolver (D2): the nearest binding of a name, by Rust's block scope.
//!
//! One resolver serves every rule that names a binding. It reads a path of [`Step`]s (the file
//! down to the place of the use) and looks upward: the statements of a block before the one the
//! path goes through, the pattern of a `for` loop or a closure for its body, the patterns of an
//! `if let` or `while let` chain for its body, an arm's pattern, and a function's parameters, where
//! the search ends. Items declared in a block do not shadow here: the drain rule asks
//! [`super::items::names_item`] whether an item in the scope of each binding that a rule read can name it
//! (D9, root Amendment 4).

use syn::visit::{self, Visit};

use super::names::{bare, tokens_name};
use super::shapes::simple_binding;
use super::tree::{Node, Step, Via};

/// What a name resolved to.
#[derive(Clone, Copy)]
pub(super) struct Binding<'f> {
    /// The binding's identity: the kind and address of the node that introduced the name.
    id: (u8, usize),
    pub(super) kind: BindingKind<'f>,
}

#[derive(Clone, Copy)]
pub(super) enum BindingKind<'f> {
    /// A `let` statement.
    Let {
        stmt: &'f syn::Stmt,
        local: &'f syn::Local,
    },
    /// A function parameter. `ty` is its type when the parameter is one plain name (`name` or
    /// `mut name`), and `None` for a receiver or a destructuring pattern.
    Param { ty: Option<&'f syn::Type> },
    /// The pattern of a `for`, `if let`, `while let`, `match` arm or closure. `owner` is the node
    /// whose pattern binds the name: the name is in scope in all of it.
    Pattern { owner: Node<'f> },
}

impl Binding<'_> {
    /// Whether both are the same binding.
    pub(super) fn same(&self, other: &Self) -> bool {
        self.id == other.id
    }

    /// Whether the name is bound by the pattern of this `for` loop.
    pub(super) fn is_pattern_of(&self, each: &syn::ExprForLoop) -> bool {
        matches!(self.kind, BindingKind::Pattern { .. })
            && self.id == (7, std::ptr::from_ref(each) as usize)
    }
}

/// The nearest binding of `name` for a use at the end of `steps`, or `None` when no binding of
/// the function (or closure chain) holds it.
pub(super) fn nearest<'f>(steps: &[Step<'f>], name: &str) -> Option<Binding<'f>> {
    for step in steps.iter().rev() {
        let found = match (step.node, step.via) {
            (Node::Block(block), Some(Via::Stmt(index))) => block.stmts[..index]
                .iter()
                .rev()
                .find_map(|stmt| match stmt {
                    syn::Stmt::Local(local) if pat_binds(&local.pat, name) => Some(Binding {
                        id: (10, std::ptr::from_ref(local) as usize),
                        kind: BindingKind::Let { stmt, local },
                    }),
                    _ => None,
                }),
            (Node::Expr(expr), Some(via)) => expression_binding(expr, via, name),
            (Node::Arm(arm), Some(Via::Guard | Via::Body)) => {
                let guard = arm
                    .guard
                    .as_ref()
                    .is_some_and(|(_, guard)| chain_binds(guard, name));
                (pat_binds(&arm.pat, name) || (guard && matches!(step.via, Some(Via::Body))))
                    .then(|| pattern(8, std::ptr::from_ref(arm) as usize, Node::Arm(arm)))
            }
            (Node::Item(syn::Item::Fn(function)), Some(Via::Body)) => {
                return parameter(&function.sig, name);
            }
            (Node::ImplFn(function), Some(Via::Body)) => return parameter(&function.sig, name),
            (Node::TraitFn(function), Some(Via::Body)) => return parameter(&function.sig, name),
            _ => None,
        };
        if found.is_some() {
            return found;
        }
    }
    None
}

fn pattern<'f>(kind: u8, address: usize, owner: Node<'f>) -> Binding<'f> {
    Binding {
        id: (kind, address),
        kind: BindingKind::Pattern { owner },
    }
}

/// The binding that an expression introduces for the slot `via` of itself.
fn expression_binding<'f>(expr: &'f syn::Expr, via: Via, name: &str) -> Option<Binding<'f>> {
    match (expr, via) {
        (syn::Expr::ForLoop(node), Via::Body) => pat_binds(&node.pat, name)
            .then(|| pattern(7, std::ptr::from_ref(node) as usize, Node::Expr(expr))),
        (syn::Expr::Closure(node), Via::Body) => node
            .inputs
            .iter()
            .any(|input| pat_binds(input, name))
            .then(|| pattern(7, std::ptr::from_ref(node) as usize, Node::Expr(expr))),
        // A `let` chain binds in the rest of its condition and in the body: read it as binding
        // everywhere in both, which can only refuse more.
        (syn::Expr::While(node), Via::Body | Via::Header) => chain_binds(&node.cond, name)
            .then(|| pattern(7, std::ptr::from_ref(node) as usize, Node::Expr(expr))),
        (syn::Expr::If(node), Via::Then | Via::Cond) => chain_binds(&node.cond, name)
            .then(|| pattern(7, std::ptr::from_ref(node) as usize, Node::Expr(expr))),
        _ => None,
    }
}

fn parameter<'f>(signature: &'f syn::Signature, name: &str) -> Option<Binding<'f>> {
    signature.inputs.iter().find_map(|input| match input {
        syn::FnArg::Receiver(receiver) if name == "self" => Some(Binding {
            id: (11, std::ptr::from_ref(receiver) as usize),
            kind: BindingKind::Param { ty: None },
        }),
        syn::FnArg::Typed(typed) if pat_binds(&typed.pat, name) => Some(Binding {
            id: (11, std::ptr::from_ref(typed) as usize),
            kind: BindingKind::Param {
                ty: (simple_binding(&typed.pat).as_deref() == Some(name)).then_some(&*typed.ty),
            },
        }),
        _ => None,
    })
}

/// Whether the pattern binds `name`. An identifier pattern may be a constant; a macro pattern may
/// bind anything it names: both count.
pub(super) fn pat_binds(pat: &syn::Pat, name: &str) -> bool {
    let mut finder = PatBinds { name, found: false };
    finder.visit_pat(pat);
    finder.found
}

/// Whether any `let` condition inside `cond` (a `&&` chain, possibly nested) binds `name`.
fn chain_binds(cond: &syn::Expr, name: &str) -> bool {
    let mut finder = LetBinds { name, found: false };
    finder.visit_expr(cond);
    finder.found
}

struct PatBinds<'a> {
    name: &'a str,
    found: bool,
}

impl<'ast> Visit<'ast> for PatBinds<'_> {
    fn visit_pat_ident(&mut self, node: &'ast syn::PatIdent) {
        if bare(&node.ident) == self.name {
            self.found = true;
        }
        visit::visit_pat_ident(self, node);
    }

    fn visit_pat(&mut self, node: &'ast syn::Pat) {
        let tokens = match node {
            syn::Pat::Verbatim(tokens) => Some(tokens),
            syn::Pat::Macro(call) => Some(&call.mac.tokens),
            _ => None,
        };
        if tokens.is_some_and(|tokens| tokens_name(tokens, self.name)) {
            self.found = true;
        }
        visit::visit_pat(self, node);
    }
}

struct LetBinds<'a> {
    name: &'a str,
    found: bool,
}

impl<'ast> Visit<'ast> for LetBinds<'_> {
    fn visit_expr_let(&mut self, node: &'ast syn::ExprLet) {
        if pat_binds(&node.pat, self.name) {
            self.found = true;
        }
        visit::visit_expr_let(self, node);
    }
}
