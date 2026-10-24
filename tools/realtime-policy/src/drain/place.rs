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
//! alias taken before the count. #1418 reuses this rule for the base of a selecting loop.

use super::Ctx;
use super::facts::UseKind;
use super::names::bare;
use super::scope::{Binding, nearest};
use super::shapes::{Root, RootKind, Seg};
use super::tree::{Node, Pos, Step, Via};

/// Whether no use of `root`, bound by `binding`, in `from..to` replaces the place it starts.
pub(super) fn untouched<'f>(
    ctx: &Ctx<'f>,
    root: &Root,
    binding: &Binding<'f>,
    from: Pos,
    to: Pos,
) -> bool {
    ctx.facts.uses_in(&root.name, from, to).iter().all(|used| {
        // A mention in a macro's token tree can be anything the macro makes of it.
        if used.kind != UseKind::Path {
            return false;
        }
        let steps = ctx.tree.descend(used.pos);
        match nearest(&steps, &root.name) {
            Some(found) if found.same(binding) => allowed(&steps, used.pos, root),
            // Another binding of the name, or none: not this place.
            _ => true,
        }
    })
}

/// Whether the use of the root at `at`, at the end of `steps`, leaves the place alone.
fn allowed(steps: &[Step<'_>], at: Pos, root: &Root) -> bool {
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
    // The place above the use: climb the `.field`, `[..]` and `*` steps.
    let mut top = steps.len() - 1;
    let mut fields = Vec::new();
    while top >= 1 {
        let (node, via) = (steps[top - 1].node, steps[top - 1].via);
        match (node, via) {
            (Node::Expr(syn::Expr::Field(field)), Some(Via::Operand(0))) => {
                fields.push(match &field.member {
                    syn::Member::Named(name) => Seg::Field(bare(name)),
                    syn::Member::Unnamed(index) => Seg::Field(format!("#{}", index.index)),
                });
            }
            (Node::Expr(syn::Expr::Index(_)), Some(Via::Operand(0))) => fields.push(Seg::Index),
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
        (Node::Expr(syn::Expr::MethodCall(_)), Some(Via::Operand(0))) => {
            root.kind == RootKind::Index || !indexed_next(&fields, &root.path)
        }
        // An index place may be read by value; the queue's root may not.
        _ => root.kind == RootKind::Index,
    }
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

fn is_assign_op(op: &syn::BinOp) -> bool {
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
