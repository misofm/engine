//! The place rule (#1426 D1.3, root decision R1): between the count and the end of the counted
//! loop, nothing written replaces the place that the counted receiver names.
//!
//! The window runs from the start of the count (its `let`, or the loop's header) to the end of the
//! counted loop. Every use in it of a name that resolves to the binding of a receiver's root is
//! read with the `.field`, `[..]` and `*` steps above it, the place it names. A place whose fields
//! differ from the receiver's at a step both have leaves the receiver and is allowed in any
//! position. Any other place (the receiver, a prefix of it such as the bare root, or a place below
//! it) is refused when it is:
//!
//! - the target of `=` or of a compound assignment (`self.control = ..`, `*control = ..`);
//! - under an explicit `&mut` or `&raw mut` (`mem::swap(&mut self.control, ..)`, any call that
//!   takes the place);
//! - for the receiver's own root, used by value: a call or method argument (`mem::swap(control,
//!   ..)`), a `let` initializer, a scrutinee, a field, an operand, anything but the receiver of a
//!   method call or a shared `&`;
//! - named in a closure, or in a macro's token tree (a trusted `debug_assert!` is parsed, and its
//!   arguments are read like any other expression).
//!
//! A `let` or a pattern that rebinds the root is the binding check (D1.2): the pop's root then
//! resolves to another binding. An index place (`lane` of `controls[lane]`) is held to the same
//! list, except that it may be read by value.
//!
//! **Stated limit (R1, the family of root's 2026-10-24 ruling on the call rule, #1544).** A method
//! call through auto-ref on the receiver (`self.data_consumer.peek(0)`) or on a prefix of it
//! (`self.settle_block(block)`) is allowed: the method can take `&mut self` and replace the field
//! inside its body, which this rule does not read. The one exception is a method call on the
//! collection that the receiver indexes (`self.queues.swap(0, 1)` beside a counted
//! `self.queues[0]`), which is refused. A zero-argument method call inside the receiver itself
//! (`self.next_queue().try_pop()`, counted as `self.next_queue().available_at_entry()`) is
//! compared by spelling only: the method can return another queue on each call, which this rule
//! does not read either. Neither does it read a place reached through an
//! alias taken before the count. #1418 holds the roots of every selecting outer loop to this rule
//! over the loop's body ([`super::select`]).

use super::Ctx;
use super::facts::UseKind;
use super::names::bare;
use super::scope::{Binding, nearest};
use super::shapes::{Place, Root, RootKind, Seg};
use super::tree::{Node, Pos, Step, Via};

/// Whether no use of `root`, bound by `binding`, in `from..to` replaces the place it starts.
pub(super) fn untouched<'f>(
    ctx: &Ctx<'f>,
    root: &Root,
    binding: &Binding<'f>,
    from: Pos,
    to: Pos,
) -> bool {
    untouched_except(ctx, root, binding, from, to, &[])
}

/// [`untouched`], with the uses at `own` left out: the reads of the rule that asks (#1418: the
/// root of a selecting loop's rebinding initializer, or of its collection).
pub(super) fn untouched_except<'f>(
    ctx: &Ctx<'f>,
    root: &Root,
    binding: &Binding<'f>,
    from: Pos,
    to: Pos,
    own: &[Pos],
) -> bool {
    // The receiver's own index expressions, by step of its place.
    let receiver_steps = ctx.tree.descend(root.at);
    let (_, _, receiver_indexes) = climb(&receiver_steps);
    ctx.facts.uses_in(&root.name, from, to).iter().all(|used| {
        if used.kind == UseKind::Path && own.contains(&used.pos) {
            return true;
        }
        // A mention in a macro's token tree can be anything the macro makes of it.
        if used.kind != UseKind::Path {
            return false;
        }
        let steps = ctx.tree.descend(used.pos);
        match nearest(&steps, &root.name) {
            Some(found) if found.same(binding) => {
                allowed(&steps, used.pos, root, &receiver_indexes)
            }
            // Another binding of the name, or none: not this place.
            _ => true,
        }
    })
}

/// Whether the use of the root at `at`, at the end of `steps`, leaves the place alone.
/// `receiver_indexes` are the index expressions of the receiver's own place, by step.
fn allowed(
    steps: &[Step<'_>],
    at: Pos,
    root: &Root,
    receiver_indexes: &[Option<&syn::Expr>],
) -> bool {
    if steps
        .iter()
        .any(|step| matches!(step.node, Node::Expr(syn::Expr::Closure(_))))
    {
        return false;
    }
    let Some(Step {
        node: Node::Expr(syn::Expr::Path(path)),
        ..
    }) = steps.last()
    else {
        // A use the walk cannot reach is refused: only a fail-closed answer is safe.
        return false;
    };
    if super::shapes::simple_path(path).is_none_or(|(_, found)| found != at) {
        return false;
    }
    let (top, fields, indexes) = climb(steps);
    if leaves(&fields, &root.path) {
        return true;
    }
    let Some(parent) = top.checked_sub(1).map(|index| steps[index]) else {
        return false;
    };
    match (parent.node, parent.via) {
        (Node::Expr(syn::Expr::Assign(_)), Some(Via::Operand(0))) => false,
        (Node::Expr(syn::Expr::Binary(binary)), Some(Via::Operand(0)))
            if is_assign_op(&binary.op) =>
        {
            false
        }
        (Node::Expr(syn::Expr::Reference(reference)), _) => reference.mutability.is_none(),
        (Node::Expr(syn::Expr::RawAddr(raw)), _) => {
            !matches!(raw.mutability, syn::PointerMutability::Mut(_))
        }
        // A method call through auto-ref: the stated limit, except on the collection that the
        // receiver indexes (`self.queues.swap(0, 1)` beside `self.queues[0]`), which reorders
        // the queues under the index.
        // A method call on a place that indexes by anything but the receiver's own index
        // (`controls[..].swap(0, 1)`, `controls[0..2].swap(0, 1)` beside `controls[lane]`)
        // reorders the queues under the index too (#1418 R3).
        (Node::Expr(syn::Expr::MethodCall(_)), Some(Via::Operand(0))) => {
            root.kind == RootKind::Index
                || (!indexed_next(&fields, &root.path) && own_indexes(&indexes, receiver_indexes))
        }
        // An index place may be read by value; the queue's root may not.
        _ => root.kind == RootKind::Index,
    }
}

/// The place above the path use that ends `steps`: climb the `.field`, `[..]` and `*` steps. The
/// index of the parent node, the place's steps, and each step's index expression (`None` for a
/// field).
fn climb<'f>(steps: &[Step<'f>]) -> (usize, Vec<Seg>, Vec<Option<&'f syn::Expr>>) {
    let mut top = steps.len().saturating_sub(1);
    let mut fields = Vec::new();
    let mut indexes = Vec::new();
    while top >= 1 {
        let (node, via) = (steps[top - 1].node, steps[top - 1].via);
        match (node, via) {
            (Node::Expr(syn::Expr::Field(field)), Some(Via::Operand(0))) => {
                fields.push(match &field.member {
                    syn::Member::Named(name) => Seg::Field(bare(name)),
                    syn::Member::Unnamed(index) => Seg::Field(format!("#{}", index.index)),
                });
                indexes.push(None);
            }
            (Node::Expr(syn::Expr::Index(index)), Some(Via::Operand(0))) => {
                fields.push(Seg::Index);
                indexes.push(Some(&*index.index));
            }
            (
                Node::Expr(syn::Expr::Unary(syn::ExprUnary {
                    op: syn::UnOp::Deref(_),
                    ..
                })),
                Some(Via::Operand(0)),
            ) => {}
            _ => break,
        }
        top -= 1;
    }
    (top, fields, indexes)
}

/// Whether every index step of a place is the receiver's own index at that step, spelled the same:
/// the same integer literal or the same [`Place`]. A range or any other expression is not.
fn own_indexes(place: &[Option<&syn::Expr>], receiver: &[Option<&syn::Expr>]) -> bool {
    place.iter().enumerate().all(|(at, index)| {
        let Some(index) = index else {
            return true;
        };
        let Some(Some(own)) = receiver.get(at) else {
            return false;
        };
        match (index, own) {
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
                (Place::of(index), Place::of(own)),
                (Some(a), Some(b)) if a.spelled_like(&b)
            ),
        }
    })
}

/// Whether `place` is a prefix of the receiver's path that an index step of it directly follows.
fn indexed_next(place: &[Seg], receiver: &[Seg]) -> bool {
    receiver.get(place.len()) == Some(&Seg::Index) && receiver.starts_with(place)
}

/// Whether a place path differs from the receiver's at a named field that both have. An index
/// step differs from nothing.
fn leaves(place: &[Seg], receiver: &[Seg]) -> bool {
    place
        .iter()
        .zip(receiver)
        .any(|pair| matches!(pair, (Seg::Field(a), Seg::Field(b)) if a != b))
}

/// Whether `op` is a compound assignment (`+=`, `-=`, ..), which replaces its left operand.
pub(super) fn is_assign_op(op: &syn::BinOp) -> bool {
    matches!(
        op,
        syn::BinOp::AddAssign(_)
            | syn::BinOp::SubAssign(_)
            | syn::BinOp::MulAssign(_)
            | syn::BinOp::DivAssign(_)
            | syn::BinOp::RemAssign(_)
            | syn::BinOp::BitXorAssign(_)
            | syn::BinOp::BitAndAssign(_)
            | syn::BinOp::BitOrAssign(_)
            | syn::BinOp::ShlAssign(_)
            | syn::BinOp::ShrAssign(_)
    )
}
