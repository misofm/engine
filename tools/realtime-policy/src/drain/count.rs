//! The counted loop (D6, D7, D8): a loop whose pass pops at most the records counted at entry.
//!
//! A loop bounds the pops in its body when its whole header is `for P in 0..<count>`,
//! `for P in 0..<recv>.available_at_entry()`, or `while <count> != 0` (or `> 0`) with a body that
//! opens with `<count> -= 1;`. `<count>` is the nearest binding of its name (D2): a `let` that
//! is `<recv>.available_at_entry()[.min(<cap>)]` (D6), or one plain alias of such a `let`, with
//! nothing between the binding and the loop but comparisons of the count.

use super::Ctx;
use super::facts::{Use, UseKind};
use super::names::bare;
use super::scope::{Binding, BindingKind};
use super::shapes::{entry_count, is_int, method0, path_name, receiver, simple_binding};
use super::tree::{Node, Pos, Step, Via, pos};

/// Whether the loop at `steps[at]` bounds the pops in its body (D8).
pub(super) fn bounds<'f>(ctx: &Ctx<'f>, steps: &[Step<'f>], at: usize) -> bool {
    let mark = ctx.mark();
    let bounded = match steps[at].node {
        Node::Expr(syn::Expr::ForLoop(each)) => for_bounds(ctx, each),
        Node::Expr(syn::Expr::While(cond)) => while_bounds(ctx, cond),
        _ => false,
    };
    // A loop that does not bound its pops read nothing (D10).
    if !bounded {
        ctx.rewind(mark);
    }
    bounded
}

/// Where a loop's header starts: its label, or its keyword.
pub(super) fn header_start(label: Option<&syn::Label>, keyword: proc_macro2::Span) -> Pos {
    pos(label.map_or(keyword, |label| label.name.apostrophe).start())
}

fn for_bounds<'f>(ctx: &Ctx<'f>, each: &'f syn::ExprForLoop) -> bool {
    let syn::Expr::Range(range) = &*each.expr else {
        return false;
    };
    if !matches!(range.limits, syn::RangeLimits::HalfOpen(_)) {
        return false;
    }
    let (Some(start), Some(end)) = (&range.start, &range.end) else {
        return false;
    };
    if !is_int(start, "0") {
        return false;
    }
    if let Some(call) = method0(end, "available_at_entry") {
        return receiver(&call.receiver);
    }
    let Some((name, at)) = path_name(end) else {
        return false;
    };
    let header = (
        header_start(each.label.as_ref(), each.for_token.span),
        pos(each.body.brace_token.span.open().start()),
    );
    counted(ctx, &name, at, header).is_some()
}

fn while_bounds<'f>(ctx: &Ctx<'f>, each: &'f syn::ExprWhile) -> bool {
    let syn::Expr::Binary(cond) = &*each.cond else {
        return false;
    };
    if !matches!(cond.op, syn::BinOp::Ne(_) | syn::BinOp::Gt(_)) || !is_int(&cond.right, "0") {
        return false;
    }
    let Some((name, at)) = path_name(&cond.left) else {
        return false;
    };
    let header = (
        header_start(each.label.as_ref(), each.while_token.span),
        pos(each.body.brace_token.span.open().start()),
    );
    let Some(binding) = counted(ctx, &name, at, header) else {
        return false;
    };
    // The body opens with `<count> -= 1;` and never names the count again.
    let Some(first) = each.body.stmts.first() else {
        return false;
    };
    let syn::Stmt::Expr(syn::Expr::Binary(decrement), Some(_)) = first else {
        return false;
    };
    let Some((first_name, first_at)) = path_name(&decrement.left) else {
        return false;
    };
    if first_name != name || !matches!(decrement.op, syn::BinOp::SubAssign(_)) {
        return false;
    }
    if !is_int(&decrement.right, "1") {
        return false;
    }
    let Some(first_extent) = ctx.tree.extent(Node::Stmt(first)) else {
        return false;
    };
    let body_close = pos(each.body.brace_token.span.close().end());
    let named = ctx.resolved_uses(&name, first_extent.core, body_close, &binding);
    named.len() == 1 && named[0].pos == first_at
}

/// Whether `name`, used at `at` in a loop header spanning `header`, is an entry count (D6, D7):
/// the binding it resolves to is returned.
fn counted<'f>(ctx: &Ctx<'f>, name: &str, at: Pos, header: (Pos, Pos)) -> Option<Binding<'f>> {
    let binding = ctx.binding_of_path(name, at)?;
    let BindingKind::Let { stmt, .. } = binding.kind else {
        return None;
    };
    // The header names the count exactly once: as a token in any role, and as one use.
    if ctx.boundaries.words_in(name, header.0, header.1) != 1
        || ctx.resolved_uses(name, header.0, header.1, &binding).len() != 1
    {
        return None;
    }
    let bound = ctx.tree.extent(Node::Stmt(stmt))?;
    if !ctx.region.holds(bound.core, bound.end) || !entry_binding(ctx, &binding, 0) {
        return None;
    }
    only_reads(ctx, name, &binding, bound.end, header.0).then(|| {
        ctx.note_read(name, binding);
        binding
    })
}

/// D6: a `let` of the count in one of the entry forms, or (at `depth` 0) one plain alias of one.
fn entry_binding<'f>(ctx: &Ctx<'f>, binding: &Binding<'f>, depth: usize) -> bool {
    let BindingKind::Let { stmt, local } = binding.kind else {
        return false;
    };
    let Some(init) = &local.init else {
        return false;
    };
    if init.diverge.is_some() || simple_binding(&local.pat).is_none() || under_cfg(local) {
        return false;
    }
    if entry_count(&init.expr) {
        return true;
    }
    let Some((entry_name, entry_at)) = path_name(&init.expr) else {
        return false;
    };
    if depth != 0 {
        return false;
    }
    let Some(entry) = ctx.binding_of_path(&entry_name, entry_at) else {
        return false;
    };
    let BindingKind::Let {
        stmt: entry_stmt, ..
    } = entry.kind
    else {
        return false;
    };
    let (Some(alias), Some(first)) = (
        ctx.tree.extent(Node::Stmt(stmt)),
        ctx.tree.extent(Node::Stmt(entry_stmt)),
    ) else {
        return false;
    };
    let held = ctx.region.holds(first.core, first.end)
        && ctx
            .resolved_uses(&entry_name, alias.core, alias.end, &entry)
            .len()
            == 1
        && only_reads(ctx, &entry_name, &entry, first.end, alias.core)
        && entry_binding(ctx, &entry, 1);
    if held {
        ctx.note_read(&entry_name, entry);
    }
    held
}

/// D2: whether the `let` carries a `cfg`-family attribute (`cfg`, `cfg_attr`, or a path ending in
/// either). A binding that may not be compiled leaves an earlier binding of the name as the count.
fn under_cfg(local: &syn::Local) -> bool {
    local.attrs.iter().any(|attribute| {
        attribute
            .path()
            .segments
            .last()
            .is_some_and(|last| matches!(bare(&last.ident).as_str(), "cfg" | "cfg_attr"))
    })
}

/// D7: every use of the binding in `from..to` is the left operand of a comparison.
fn only_reads<'f>(ctx: &Ctx<'f>, name: &str, binding: &Binding<'f>, from: Pos, to: Pos) -> bool {
    ctx.resolved_uses(name, from, to, binding)
        .iter()
        .all(|used| is_read(ctx, used))
}

/// Whether the use is the left operand of `==`, `!=`, `<`, `<=`, `>` or `>=`. A use in a macro's
/// token tree is not (a `debug_assert!` is parsed, so its uses are paths).
fn is_read<'f>(ctx: &Ctx<'f>, used: &Use) -> bool {
    if used.kind != UseKind::Path {
        return false;
    }
    let steps = ctx.tree.descend(used.pos);
    let [.., parent, leaf] = steps.as_slice() else {
        return false;
    };
    matches!(
        (parent.node, parent.via, leaf.node),
        (
            Node::Expr(syn::Expr::Binary(syn::ExprBinary {
                op: syn::BinOp::Eq(_)
                    | syn::BinOp::Ne(_)
                    | syn::BinOp::Lt(_)
                    | syn::BinOp::Le(_)
                    | syn::BinOp::Gt(_)
                    | syn::BinOp::Ge(_),
                ..
            })),
            Some(Via::Operand(0)),
            Node::Expr(syn::Expr::Path(_)),
        )
    )
}
