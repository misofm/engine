//! Condition 6 of the per-queue form: nothing between the set loop and the end of the form's
//! outermost loop writes the counters or replaces the queues.
//!
//! The window runs from the end of the set loop to the end of the form's outermost loop. Every use
//! in it of a name that resolves to the root binding of `<K>` or `<Q>` must be one of five: a field
//! path that leaves both, the pass loop's own header, a shared read of `<Q>`, a read loop over
//! `<K>`, or a terminal call. Any other use refuses the form, inside closures and macro token trees
//! too.

use super::super::Ctx;
use super::super::facts::{POP, Use, UseKind};
use super::super::names::bare;
use super::super::scope::nearest;
use super::super::shapes::{Place, simple_path};
use super::super::tree::{Node, Step, Via, pos};
use super::{Located, Pass, leaf};

/// Whether every use of a root in the window is allowed.
pub(super) fn holds<'f>(pass: &Pass<'_, 'f>, ctx: &Ctx<'f>, found: &Located<'_, 'f>) -> bool {
    let steps = found.steps;
    // The form's outermost loop: the first loop below the block that holds the set loop.
    let Some(outer) = (found.block_at + 1..=pass.at).find(|&index| steps[index].node.is_loop())
    else {
        return false;
    };
    let Some(outer_extent) = ctx.tree.extent(steps[outer].node) else {
        return false;
    };
    let Some(function) = steps[..=pass.at]
        .iter()
        .rposition(|step| step.node.is_function())
    else {
        return false;
    };
    let window = Window {
        pass,
        found,
        outer,
        function: steps[function].node.id(),
    };
    let mut names = vec![pass.q.name.as_str()];
    if pass.k.name != pass.q.name {
        names.push(pass.k.name.as_str());
    }
    names.into_iter().all(|name| {
        ctx.facts
            .uses_in(name, found.set_end, outer_extent.end)
            .iter()
            .all(|used| window.allows(ctx, name, used))
    })
}

/// Condition 6's window: from the end of the set loop to the end of the form's outermost loop.
struct Window<'a, 'b, 'f> {
    pass: &'a Pass<'b, 'f>,
    found: &'a Located<'a, 'f>,
    /// The step of the form's outermost loop.
    outer: usize,
    /// The function that holds the form.
    function: (u8, usize),
}

impl<'f> Window<'_, '_, 'f> {
    /// Whether this use of a root name is allowed: it is not a root, or it is one of the five.
    fn allows(&self, ctx: &Ctx<'f>, name: &str, used: &Use) -> bool {
        let steps = ctx.tree.descend(used.pos);
        let Some(binding) = nearest(&steps, name) else {
            return true;
        };
        let is_queue = binding.same(self.found.queue_root);
        let is_counter = binding.same(self.found.counter_root);
        if !is_queue && !is_counter {
            return true;
        }
        if used.kind != UseKind::Path {
            return false;
        }
        let Some(Step {
            node: Node::Expr(syn::Expr::Path(path)),
            ..
        }) = steps.last().copied()
        else {
            return false;
        };
        if simple_path(path).is_none_or(|(_, at)| at != used.pos) {
            return false;
        }
        // The field path after the root: climb the `.field` steps above the use.
        let mut top = steps.len() - 1;
        let mut fields = Vec::new();
        while top >= 1
            && let (Node::Expr(syn::Expr::Field(field)), Some(Via::Operand(0))) =
                (steps[top - 1].node, steps[top - 1].via)
        {
            fields.push(match &field.member {
                syn::Member::Named(name) => bare(name),
                syn::Member::Unnamed(index) => format!("#{}", index.index),
            });
            top -= 1;
        }
        let q = is_queue.then_some(&self.found.set.q);
        let k = is_counter.then_some(&self.found.set.k);
        let targets: Vec<&Place<'f>> = q.into_iter().chain(k).collect();
        // Item 1: a field path that leaves `<K>` and `<Q>`.
        if targets
            .iter()
            .all(|target| diverges(&fields, &target.fields))
        {
            return true;
        }
        // Item 2: the pass loop's own header.
        if std::ptr::eq(path, self.pass.q.root) || std::ptr::eq(path, self.pass.k.root) {
            return true;
        }
        // Item 3: a shared read of `<Q>`.
        if is_queue && fields == self.found.set.q.fields && shared_read(&steps, top) {
            return true;
        }
        // Item 4: a read loop over `<K>`.
        if is_counter && fields == self.found.set.k.fields && self.read_loop(ctx, &steps, top) {
            return true;
        }
        // Item 5: the terminal call.
        targets.iter().any(|target| {
            fields.len() <= target.fields.len() && fields == target.fields[..fields.len()]
        }) && self.terminal_call(&steps, top)
    }

    /// Item 4: `for <x> in <K>.iter()` or `for <x> in &<K>`, whose body names `<x>` only as `*<x>`
    /// that is not assigned to, not under `&mut`, and not in a closure or a macro.
    fn read_loop(&self, ctx: &Ctx<'f>, steps: &[Step<'f>], top: usize) -> bool {
        let Some(parent) = top.checked_sub(1) else {
            return false;
        };
        let wrapper_ok = match (steps[parent].node, steps[parent].via) {
            (Node::Expr(syn::Expr::MethodCall(call)), Some(Via::Operand(0))) => {
                call.turbofish.is_none() && call.args.is_empty() && bare(&call.method) == "iter"
            }
            (Node::Expr(syn::Expr::Reference(reference)), Some(Via::Operand(0))) => {
                reference.mutability.is_none()
            }
            _ => false,
        };
        if !wrapper_ok || parent == 0 {
            return false;
        }
        let Step {
            node: Node::Expr(syn::Expr::ForLoop(each)),
            via: Some(Via::Header),
        } = steps[parent - 1]
        else {
            return false;
        };
        let Some(name) = leaf(&each.pat) else {
            return false;
        };
        let body = (
            pos(each.body.brace_token.span.open().start()),
            pos(each.body.brace_token.span.close().end()),
        );
        ctx.facts.uses_in(&name, body.0, body.1).iter().all(|used| {
            let at = ctx.tree.descend(used.pos);
            if nearest(&at, &name).is_none_or(|binding| !binding.is_pattern_of(each)) {
                return true;
            }
            plain_deref_read(&at, used, each)
        })
    }

    /// Item 5: a method-call statement on a root or a prefix of `<K>` or `<Q>`, directly in a
    /// block of the function body, after which the block holds no loop, no pop, no closure and no
    /// macro call, and ends in `return` or a `break` out of the form's outermost loop.
    fn terminal_call(&self, steps: &[Step<'f>], top: usize) -> bool {
        // The use's receiver chain, the call, the statement and its block.
        if top < 3 {
            return false;
        }
        let (Node::Expr(syn::Expr::MethodCall(_)), Some(Via::Operand(0))) =
            (steps[top - 1].node, steps[top - 1].via)
        else {
            return false;
        };
        let (Node::Stmt(syn::Stmt::Expr(_, Some(_))), Some(Via::Operand(0))) =
            (steps[top - 2].node, steps[top - 2].via)
        else {
            return false;
        };
        let (Node::Block(block), Some(Via::Stmt(index))) =
            (steps[top - 3].node, steps[top - 3].via)
        else {
            return false;
        };
        // In the form's own function, not in a closure, a macro or a nested function.
        let Some(function) = steps[..top]
            .iter()
            .rposition(|step| step.node.is_function())
        else {
            return false;
        };
        if steps[function].node.id() != self.function {
            return false;
        }
        if steps[function + 1..top - 2].iter().any(|step| {
            matches!(step.node, Node::Expr(syn::Expr::Closure(_)))
                || step.node.macro_call().is_some()
                || matches!(step.node, Node::Item(_))
        }) {
            return false;
        }
        let tail = &block.stmts[index + 1..];
        let Some(last) = tail.last() else {
            return false;
        };
        let mut banned = Banned::default();
        for stmt in tail {
            syn::visit::Visit::visit_stmt(&mut banned, stmt);
        }
        if banned.found {
            return false;
        }
        match last {
            syn::Stmt::Expr(syn::Expr::Return(_), _) => true,
            syn::Stmt::Expr(syn::Expr::Break(exit), _) => {
                self.breaks_out_of_outer(steps, top - 3, exit.label.as_ref())
            }
            _ => false,
        }
    }

    /// Whether a `break` (with `label`) standing in the block at `steps[block]` leaves the form's
    /// outermost loop.
    fn breaks_out_of_outer(
        &self,
        steps: &[Step<'f>],
        block: usize,
        label: Option<&syn::Lifetime>,
    ) -> bool {
        for index in (0..block).rev() {
            let Node::Expr(expr) = steps[index].node else {
                continue;
            };
            let own = match expr {
                syn::Expr::ForLoop(node) => node.label.as_ref(),
                syn::Expr::While(node) => node.label.as_ref(),
                syn::Expr::Loop(node) => node.label.as_ref(),
                syn::Expr::Block(node) => {
                    // A labeled block can be a `break` target too, and it is not a loop.
                    if let Some(wanted) = label
                        && node
                            .label
                            .as_ref()
                            .is_some_and(|own| own.name.ident == wanted.ident)
                    {
                        return false;
                    }
                    continue;
                }
                _ => continue,
            };
            let targeted = match label {
                None => true,
                Some(wanted) => own.is_some_and(|own| own.name.ident == wanted.ident),
            };
            if targeted {
                return index == self.outer;
            }
        }
        false
    }
}

/// Whether two field paths differ at a step they both have.
fn diverges(left: &[String], right: &[String]) -> bool {
    left.iter().zip(right).any(|(a, b)| a != b)
}

/// Item 3: `<Q>.get(..)`, `.iter()`, `.len()`, `.is_empty()`, `.first()`, `.last()` or `&<Q>`.
fn shared_read(steps: &[Step<'_>], top: usize) -> bool {
    let Some(parent) = top.checked_sub(1) else {
        return false;
    };
    match (steps[parent].node, steps[parent].via) {
        (Node::Expr(syn::Expr::MethodCall(call)), Some(Via::Operand(0))) => {
            call.turbofish.is_none()
                && matches!(
                    bare(&call.method).as_str(),
                    "get" | "iter" | "len" | "is_empty" | "first" | "last"
                )
        }
        (Node::Expr(syn::Expr::Reference(reference)), Some(Via::Operand(0))) => {
            reference.mutability.is_none()
        }
        _ => false,
    }
}

/// A use of a read loop's element that is only `*<x>`, not assigned to, not under `&mut`, and not
/// in a closure or a macro.
fn plain_deref_read(at: &[Step<'_>], used: &Use, each: &syn::ExprForLoop) -> bool {
    if used.kind != UseKind::Path {
        return false;
    }
    let Some(own) = at
        .iter()
        .position(|step| matches!(step.node, Node::Expr(syn::Expr::ForLoop(found)) if std::ptr::eq(found, each)))
    else {
        return false;
    };
    let len = at.len();
    if len < own + 3 {
        return false;
    }
    // The use is a path directly under a dereference.
    let (Node::Expr(syn::Expr::Unary(unary)), Some(Via::Operand(0))) =
        (at[len - 2].node, at[len - 2].via)
    else {
        return false;
    };
    if !matches!(unary.op, syn::UnOp::Deref(_)) {
        return false;
    }
    at[own + 1..len - 1]
        .iter()
        .all(|step| match (step.node, step.via) {
            (Node::Expr(syn::Expr::Closure(_)), _) => false,
            (Node::Expr(syn::Expr::Reference(reference)), _) => reference.mutability.is_none(),
            (Node::Expr(syn::Expr::RawAddr(raw)), _) => {
                !matches!(raw.mutability, syn::PointerMutability::Mut(_))
            }
            (Node::Expr(syn::Expr::Assign(_)), Some(Via::Operand(0))) => false,
            (Node::Expr(syn::Expr::Binary(binary)), Some(Via::Operand(0))) => {
                !is_assign_op(&binary.op)
            }
            (node, _) => node.macro_call().is_none(),
        })
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

/// Finds a loop, a pop, a closure or a macro call.
#[derive(Default)]
struct Banned {
    found: bool,
}

impl<'ast> syn::visit::Visit<'ast> for Banned {
    fn visit_expr_for_loop(&mut self, _: &'ast syn::ExprForLoop) {
        self.found = true;
    }

    fn visit_expr_while(&mut self, _: &'ast syn::ExprWhile) {
        self.found = true;
    }

    fn visit_expr_loop(&mut self, _: &'ast syn::ExprLoop) {
        self.found = true;
    }

    fn visit_expr_closure(&mut self, _: &'ast syn::ExprClosure) {
        self.found = true;
    }

    fn visit_macro(&mut self, _: &'ast syn::Macro) {
        self.found = true;
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if bare(&node.method) == POP {
            self.found = true;
        }
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        if node
            .path
            .segments
            .last()
            .is_some_and(|last| bare(&last.ident) == POP)
        {
            self.found = true;
        }
        syn::visit::visit_expr_path(self, node);
    }
}
