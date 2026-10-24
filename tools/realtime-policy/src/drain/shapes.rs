//! Small recognizers for the expression shapes that the drain rule accepts.

use super::names::bare;
use super::tree::{Pos, pos};

/// The name and place of a single-segment path expression (`count`, `self`), if it is one.
pub(super) fn path_name(expr: &syn::Expr) -> Option<(String, Pos)> {
    let syn::Expr::Path(path) = expr else {
        return None;
    };
    simple_path(path)
}

/// The name and place of a path expression that is one plain identifier.
pub(super) fn simple_path(path: &syn::ExprPath) -> Option<(String, Pos)> {
    if path.qself.is_some() || path.path.leading_colon.is_some() || path.path.segments.len() != 1 {
        return None;
    }
    let segment = path.path.segments.first()?;
    segment
        .arguments
        .is_none()
        .then(|| (bare(&segment.ident), pos(segment.ident.span().start())))
}

/// A path of plain identifier segments (`MAX`, `Self::LIMIT`, `a::b::C`).
fn plain_segments(path: &syn::ExprPath) -> bool {
    path.qself.is_none()
        && path.path.leading_colon.is_none()
        && path
            .path
            .segments
            .iter()
            .all(|segment| segment.arguments.is_none())
}

/// A place expression: a plain identifier followed by named fields (`self.lanes`).
pub(super) struct Place<'f> {
    pub(super) root: &'f syn::ExprPath,
    pub(super) name: String,
    pub(super) at: Pos,
    pub(super) fields: Vec<String>,
}

impl<'f> Place<'f> {
    /// The place `expr` names, or `None` when it is not a plain identifier with named fields.
    pub(super) fn of(expr: &'f syn::Expr) -> Option<Self> {
        match expr {
            syn::Expr::Path(path) => {
                let (name, at) = simple_path(path)?;
                Some(Self {
                    root: path,
                    name,
                    at,
                    fields: Vec::new(),
                })
            }
            syn::Expr::Field(field) => {
                let syn::Member::Named(member) = &field.member else {
                    return None;
                };
                let mut place = Self::of(&field.base)?;
                place.fields.push(bare(member));
                Some(place)
            }
            _ => None,
        }
    }

    /// The same place spelled the same: the same root name and the same fields.
    pub(super) fn spelled_like(&self, other: &Self) -> bool {
        self.name == other.name && self.fields == other.fields
    }
}

/// A method call named `name` with no turbofish.
pub(super) fn method<'f>(expr: &'f syn::Expr, name: &str) -> Option<&'f syn::ExprMethodCall> {
    match expr {
        syn::Expr::MethodCall(call) if call.turbofish.is_none() && bare(&call.method) == name => {
            Some(call)
        }
        _ => None,
    }
}

/// A method call named `name` with no arguments and no turbofish.
pub(super) fn method0<'f>(expr: &'f syn::Expr, name: &str) -> Option<&'f syn::ExprMethodCall> {
    method(expr, name).filter(|call| call.args.is_empty())
}

/// An integer literal of exactly this decimal value and no suffix.
pub(super) fn is_int(expr: &syn::Expr, value: &str) -> bool {
    matches!(
        expr,
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit), .. })
            if lit.suffix().is_empty() && lit.base10_digits() == value
    )
}

/// `expr` without a trailing `.min(<cap>)`.
pub(super) fn uncapped(expr: &syn::Expr) -> &syn::Expr {
    match method(expr, "min") {
        Some(capped) if capped.args.len() == 1 && cap(&capped.args[0]) => &capped.receiver,
        _ => expr,
    }
}

/// D6 (D3, root Amendment 4): the receiver that the entry count `expr` counts, `<recv>` of
/// `<recv>.available_at_entry()`, optionally capped by `.min(<cap>)` (#1426 D1.1). A function
/// value of `available_at_entry` counts nothing: the `<recv>.map_or(0, <Type>::available_at_entry)`
/// form is withdrawn.
pub(super) fn entry_receiver(expr: &syn::Expr) -> Option<&syn::Expr> {
    method0(uncapped(expr), "available_at_entry")
        .map(|call| &*call.receiver)
        .filter(|found| receiver(found))
}

/// Whether two receivers of the [`receiver`] grammar are the same expression, token for token
/// (#1426 D1.1): the same identifier, then the same fields, zero-argument method calls and
/// indexes, an index by the same literal value or by the same spelled [`Place`]. An expression
/// outside the grammar is the same as nothing.
pub(super) fn same_receiver(left: &syn::Expr, right: &syn::Expr) -> bool {
    use syn::Expr as E;
    match (left, right) {
        (E::Path(a), E::Path(b)) => {
            matches!((simple_path(a), simple_path(b)), (Some((a, _)), Some((b, _))) if a == b)
        }
        (E::Field(a), E::Field(b)) => match (&a.member, &b.member) {
            (syn::Member::Named(x), syn::Member::Named(y)) => {
                bare(x) == bare(y) && same_receiver(&a.base, &b.base)
            }
            _ => false,
        },
        (E::MethodCall(a), E::MethodCall(b)) => {
            a.turbofish.is_none()
                && b.turbofish.is_none()
                && a.args.is_empty()
                && b.args.is_empty()
                && bare(&a.method) == bare(&b.method)
                && same_receiver(&a.receiver, &b.receiver)
        }
        (E::Index(a), E::Index(b)) => {
            same_receiver(&a.expr, &b.expr) && same_index(&a.index, &b.index)
        }
        _ => false,
    }
}

fn same_index(left: &syn::Expr, right: &syn::Expr) -> bool {
    match (left, right) {
        (
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(a),
                ..
            }),
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(b),
                ..
            }),
        ) => a.base10_digits() == b.base10_digits() && a.suffix() == b.suffix(),
        _ => matches!(
            (Place::of(left), Place::of(right)),
            (Some(a), Some(b)) if a.spelled_like(&b)
        ),
    }
}

/// One step of a place path: a field by name (`#n` for a tuple field) or an index by anything.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Seg {
    Field(String),
    Index,
}

/// What a root of a receiver names (#1426 D1.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RootKind {
    /// The receiver's own root: the queue is reached from it.
    Queue,
    /// The root of a [`Place`] that indexes the receiver (`lane` of `controls[lane]`).
    Index,
}

/// A root identifier of a receiver, where it is written, and the place path it starts.
#[derive(Clone, Debug)]
pub(super) struct Root {
    pub(super) name: String,
    pub(super) at: Pos,
    pub(super) kind: RootKind,
    /// The place the root starts, up to the receiver's first method call: `[control]` of
    /// `self.control.consumer()` for the queue root, the fields of the index place for an index
    /// root.
    pub(super) path: Vec<Seg>,
}

/// The roots of a receiver of the [`receiver`] grammar, the receiver's own root first, then the
/// root of each index place in source order. `None` outside the grammar.
pub(super) fn receiver_roots(expr: &syn::Expr) -> Option<Vec<Root>> {
    let mut chain = Vec::new();
    let mut indexes = Vec::new();
    let mut at = expr;
    loop {
        match at {
            syn::Expr::Path(path) => {
                let (name, place) = simple_path(path)?;
                chain.reverse();
                // The place ends at the first method call.
                let path: Vec<Seg> = chain.into_iter().map_while(|seg| seg).collect();
                let mut roots = vec![Root {
                    name,
                    at: place,
                    kind: RootKind::Queue,
                    path,
                }];
                indexes.reverse();
                roots.extend(indexes);
                return Some(roots);
            }
            syn::Expr::Field(field) => {
                let syn::Member::Named(member) = &field.member else {
                    return None;
                };
                chain.push(Some(Seg::Field(bare(member))));
                at = &field.base;
            }
            syn::Expr::MethodCall(call) => {
                if call.turbofish.is_some() || !call.args.is_empty() {
                    return None;
                }
                chain.push(None);
                at = &call.receiver;
            }
            syn::Expr::Index(index) => {
                if !index_ok(&index.index) {
                    return None;
                }
                if let Some(place) = Place::of(&index.index) {
                    indexes.push(Root {
                        name: place.name,
                        at: place.at,
                        kind: RootKind::Index,
                        path: place.fields.into_iter().map(Seg::Field).collect(),
                    });
                }
                chain.push(Some(Seg::Index));
                at = &index.expr;
            }
            _ => return None,
        }
    }
}

/// The receiver of an entry count: an identifier, then fields, zero-argument method calls, and
/// indexes by an integer literal or a path.
pub(super) fn receiver(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Path(path) => simple_path(path).is_some(),
        syn::Expr::Field(field) => {
            matches!(field.member, syn::Member::Named(_)) && receiver(&field.base)
        }
        syn::Expr::MethodCall(call) => {
            call.turbofish.is_none() && call.args.is_empty() && receiver(&call.receiver)
        }
        syn::Expr::Index(index) => receiver(&index.expr) && index_ok(&index.index),
        _ => false,
    }
}

fn index_ok(expr: &syn::Expr) -> bool {
    matches!(
        expr,
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(_),
            ..
        })
    ) || Place::of(expr).is_some()
}

/// `<cap>`: an integer literal, or an identifier followed by `.field` or `::segment` steps.
pub(super) fn cap(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(lit),
            ..
        }) => matches!(lit.suffix(), "" | "u8" | "u16" | "u32" | "u64" | "usize"),
        syn::Expr::Path(path) => plain_segments(path),
        syn::Expr::Field(field) => {
            matches!(field.member, syn::Member::Named(_)) && cap(&field.base)
        }
        _ => false,
    }
}

/// A binding pattern for one name: `name`, `mut name`, or either with a one-identifier type.
pub(super) fn simple_binding(pat: &syn::Pat) -> Option<String> {
    match pat {
        syn::Pat::Ident(ident) if ident.by_ref.is_none() && ident.subpat.is_none() => {
            Some(bare(&ident.ident))
        }
        syn::Pat::Type(typed) => {
            let syn::Type::Path(ty) = &*typed.ty else {
                return None;
            };
            let plain = ty.qself.is_none()
                && ty.path.leading_colon.is_none()
                && ty.path.segments.len() == 1
                && ty.path.segments[0].arguments.is_none();
            if plain {
                simple_binding(&typed.pat)
            } else {
                None
            }
        }
        _ => None,
    }
}
