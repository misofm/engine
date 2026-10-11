//! Finite outer loops (D1): every loop that surrounds a counted drain, inside its region, must run
//! a finite number of passes. The gate bounds each pass, not the product of the passes, so
//! "finite" is not "small": `0..usize::MAX` around a drain that re-reads its count passes.
//!
//! A loop around the innermost bounding loop is one of:
//!
//! - a bounded loop of D8 ([`count::bounds`]);
//! - `for P in 0..<bound>`, with `<bound>` an integer literal or an identifier followed by
//!   `.field` or `::segment` steps;
//! - `&<fields>`, `&mut <fields>`, `<fields>.iter()` or `<fields>.iter_mut()`, or a slice or array
//!   parameter of the enclosing `fn` that nothing names before the loop and no `static`, `const`
//!   or `use` of the region names, each optionally followed by any number of `.enumerate()` and
//!   `.zip(<literal | fields | &fields | &mut fields>)` steps (a zip ends with its receiver).
//!
//! `while` and `loop` are refused. So is every other `for` header (`1..n`, `0..=n`, `(0..)`,
//! `.rev()`, `.take(n)`, a bare local), the over-refusals that D7 keeps. The `<bound>` of
//! `0..<bound>` is stricter than the `main` gate's `path`, which also took a zero-argument method
//! call and a bracket index (`0..self.controls.len()`, `0..a[i]`): both run code this rule cannot
//! see, so both are refused, and widening that needs a ruling (B1b-D1).
//!
//! The root of `<fields>` is a read (D9): it must resolve to a binding, and no item of the region
//! may declare its name (see [`super::shadowed_by_an_item`]).
//!
//! Not checked: a collection whose type never ends. `&mut <fields>` and `<fields>.iter_mut()` are
//! finite by their form, so a field or a local that holds an unbounded iterator (`0..`) passes.
//! See the module documentation of [`super`].

use super::Ctx;
use super::count;
use super::scope::BindingKind;
use super::shapes::{Place, cap, is_int, method, method0, path_name};
use super::tree::{Node, Step, pos};

/// Whether the loop at `steps[at]` runs a finite number of passes.
pub(super) fn finite<'f>(ctx: &Ctx<'f>, steps: &[Step<'f>], at: usize) -> bool {
    match steps[at].node {
        Node::Expr(syn::Expr::ForLoop(each)) => {
            count::bounds(ctx, steps, at)
                || range_to_a_bound(&each.expr)
                || collection(ctx, steps, at, each)
        }
        Node::Expr(syn::Expr::While(_)) => count::bounds(ctx, steps, at),
        _ => false,
    }
}

/// `0..<bound>`.
fn range_to_a_bound(expr: &syn::Expr) -> bool {
    let syn::Expr::Range(range) = expr else {
        return false;
    };
    matches!(range.limits, syn::RangeLimits::HalfOpen(_))
        && range
            .start
            .as_deref()
            .is_some_and(|start| is_int(start, "0"))
        && range.end.as_deref().is_some_and(cap)
}

/// `&<fields>`, `<fields>.iter()`, `<fields>.iter_mut()` or a slice or array parameter, with any
/// `.enumerate()` and `.zip(..)` steps after it.
fn collection<'f>(
    ctx: &Ctx<'f>,
    steps: &[Step<'f>],
    at: usize,
    each: &'f syn::ExprForLoop,
) -> bool {
    let mut base = &*each.expr;
    loop {
        if let Some(call) = method0(base, "enumerate") {
            base = &call.receiver;
        } else if let Some(call) = method(base, "zip")
            && call.args.len() == 1
            && zip_argument(&call.args[0])
        {
            base = &call.receiver;
        } else {
            break;
        }
    }
    match base {
        syn::Expr::Reference(reference) => {
            Place::of(&reference.expr).is_some_and(|place| read_root(ctx, &place))
        }
        syn::Expr::MethodCall(call) => {
            (method0(base, "iter").is_some() || method0(base, "iter_mut").is_some())
                && Place::of(&call.receiver).is_some_and(|place| read_root(ctx, &place))
        }
        syn::Expr::Path(_) => path_name(base)
            .is_some_and(|(name, name_at)| slice_parameter(ctx, steps, at, &name, name_at)),
        _ => false,
    }
}

/// D9: the loop reads the root of the collection `place` to be finite, so an item of the region
/// that declares the root's name (a unit struct with an `iter_mut` that repeats) redirects it. The
/// root must resolve to a binding, which the item rule and D10's span then cover. `self` is a
/// keyword: nothing can shadow it.
fn read_root<'f>(ctx: &Ctx<'f>, place: &Place<'f>) -> bool {
    if place.name == "self" {
        return true;
    }
    let Some(binding) = ctx.binding_of_path(&place.name, place.at) else {
        return false;
    };
    ctx.note_read(&place.name, binding);
    true
}

/// A zip's argument: an integer literal, `<fields>`, `&<fields>` or `&mut <fields>`.
fn zip_argument(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(_),
            ..
        }) => true,
        syn::Expr::Reference(reference) => Place::of(&reference.expr).is_some(),
        other => Place::of(other).is_some(),
    }
}

/// Whether `name`, used at `name_at` as the iterator of the loop at `steps[at]`, is a slice or
/// array parameter of the enclosing `fn`: the parameter is a plain name of type `&[T]`, `&mut [T]`,
/// `&[T; N]` or `[T; N]`; the signature and the loop lie in the pop's region; nothing names it
/// between the `fn`'s body and the loop (a rebinding would); and no `static`, `const` or `use` of
/// the region can name it (a block item shadows the parameter for its whole block).
fn slice_parameter<'f>(
    ctx: &Ctx<'f>,
    steps: &[Step<'f>],
    at: usize,
    name: &str,
    name_at: super::tree::Pos,
) -> bool {
    let Some(binding) = ctx.binding_of_path(name, name_at) else {
        return false;
    };
    let BindingKind::Param { ty: Some(ty) } = binding.kind else {
        return false;
    };
    let finite = match ty {
        syn::Type::Array(_) => true,
        syn::Type::Reference(reference) => {
            matches!(&*reference.elem, syn::Type::Slice(_) | syn::Type::Array(_))
        }
        _ => false,
    };
    let Some(function) = steps[..at].iter().rposition(|step| step.node.is_function()) else {
        return false;
    };
    let Some(body) = body_open(steps[function].node) else {
        return false;
    };
    let (Some(signature), Some(this)) = (
        ctx.tree.extent(steps[function].node),
        ctx.tree.extent(steps[at].node),
    ) else {
        return false;
    };
    let held = finite
        && ctx.region.holds(signature.core, this.end)
        && ctx.boundaries.words_in(name, body, this.full) == 0;
    if held {
        ctx.note_read(name, binding);
    }
    held
}

/// Where the body of a function node opens.
fn body_open(node: Node<'_>) -> Option<super::tree::Pos> {
    let block = match node {
        Node::Item(syn::Item::Fn(function)) => &function.block,
        Node::ImplFn(function) => &function.block,
        Node::TraitFn(function) => function.default.as_ref()?,
        _ => return None,
    };
    Some(pos(block.brace_token.span.open().start()))
}
