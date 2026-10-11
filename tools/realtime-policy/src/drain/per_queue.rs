//! The per-queue counted pop (D8b): one remaining count per queue, read at entry, that each pop
//! is guarded by and decrements.
//!
//! The form is a set loop that reads every queue's count at entry into a counter, and a pass loop
//! that skips a queue whose counter is zero, decrements it, and pops that queue. Six conditions
//! make it bound every queue: the names resolve to the same bindings in both loops (1), the set
//! loop only reads the counts (2), the loops are the form's own (3), the pass loop pairs queue i
//! with counter i (4), its body is the guard, the decrement and one pop of the counted queue (5),
//! and nothing in the window between the loops writes the counters or replaces the queues (6).

use super::Ctx;
use super::facts::{PopKind, PopSite, Use, UseKind};
use super::names::bare;
use super::scope::{Binding, nearest};
use super::shapes::{Place, is_int, method, method0, path_name, simple_path, uncapped};
use super::tree::{Node, Pos, Step, Via, pos};

/// Whether the pop `site`, in the innermost loop `steps[at]`, is bounded by a D8b form.
pub(super) fn bounds<'f>(ctx: &Ctx<'f>, steps: &[Step<'f>], at: usize, site: &PopSite) -> bool {
    let Node::Expr(syn::Expr::ForLoop(pass)) = steps[at].node else {
        return false;
    };
    let Some(header) = PassHeader::of(&pass.expr) else {
        return false;
    };
    let Some(names) = pass_pattern(&pass.pat, header.zips.len()) else {
        return false;
    };
    (0..header.zips.len()).any(|index| {
        let form = Pass {
            each: pass,
            at,
            q: &header.q,
            k: &header.zips[index],
            r: &names[0],
            c: &names[index + 1],
        };
        form.holds(ctx, steps, site)
    })
}

/// A pass loop's iterator: `<Q>.iter_mut().zip(&mut <fields>)..`.
struct PassHeader<'f> {
    q: Place<'f>,
    /// The `.zip(&mut <fields>)` steps, innermost first.
    zips: Vec<Place<'f>>,
}

impl<'f> PassHeader<'f> {
    fn of(expr: &'f syn::Expr) -> Option<Self> {
        let mut zips = Vec::new();
        let mut current = expr;
        while let Some(call) = method(current, "zip") {
            if call.args.len() != 1 {
                return None;
            }
            zips.push(mutable_place(&call.args[0])?);
            current = &call.receiver;
        }
        zips.reverse();
        if zips.is_empty() {
            return None;
        }
        let base = method0(current, "iter_mut")?;
        Some(Self {
            q: fields_place(&base.receiver)?,
            zips,
        })
    }
}

/// `<fields>`: a plain identifier followed by one or more named fields.
fn fields_place(expr: &syn::Expr) -> Option<Place<'_>> {
    Place::of(expr).filter(|place| !place.fields.is_empty())
}

/// `&mut <fields>`.
fn mutable_place(expr: &syn::Expr) -> Option<Place<'_>> {
    match expr {
        syn::Expr::Reference(reference) if reference.mutability.is_some() => {
            fields_place(&reference.expr)
        }
        _ => None,
    }
}

/// The identifier bindings of a pass pattern, `((<b0>, <b1>), .., <bk>)` for k zip steps.
fn pass_pattern(pat: &syn::Pat, zips: usize) -> Option<Vec<String>> {
    if zips == 0 {
        return leaf(pat).map(|name| vec![name]);
    }
    let syn::Pat::Tuple(tuple) = pat else {
        return None;
    };
    if tuple.elems.len() != 2 {
        return None;
    }
    let mut names = pass_pattern(&tuple.elems[0], zips - 1)?;
    names.push(leaf(&tuple.elems[1])?);
    Some(names)
}

/// A plain identifier binding: no `ref`, no sub-pattern.
fn leaf(pat: &syn::Pat) -> Option<String> {
    match pat {
        syn::Pat::Ident(ident) if ident.by_ref.is_none() && ident.subpat.is_none() => {
            Some(bare(&ident.ident))
        }
        _ => None,
    }
}

/// A set loop: `for (<r>, <c>) in <Q>.iter().zip(&mut <K>) { *<c> = <r><path>.available_at_entry()[.min(<cap>)]; }`.
struct SetLoop<'f> {
    stmt: &'f syn::Stmt,
    each: &'f syn::ExprForLoop,
    q: Place<'f>,
    k: Place<'f>,
    r: String,
    c: String,
    /// The fields after `<r>` in the counted queue (`consumer`).
    path: Vec<String>,
    r_at: Pos,
    c_at: Pos,
}

impl<'f> SetLoop<'f> {
    fn of(stmt: &'f syn::Stmt) -> Option<Self> {
        let syn::Stmt::Expr(syn::Expr::ForLoop(each), _) = stmt else {
            return None;
        };
        let zip = method(&each.expr, "zip").filter(|zip| zip.args.len() == 1)?;
        let q = match &*zip.receiver {
            syn::Expr::Reference(reference) if reference.mutability.is_none() => {
                fields_place(&reference.expr)?
            }
            other => fields_place(&method0(other, "iter")?.receiver)?,
        };
        let k = match &zip.args[0] {
            syn::Expr::Reference(reference) if reference.mutability.is_some() => {
                fields_place(&reference.expr)?
            }
            other => fields_place(&method0(other, "iter_mut")?.receiver)?,
        };
        let syn::Pat::Tuple(tuple) = &*each.pat else {
            return None;
        };
        if tuple.elems.len() != 2 {
            return None;
        }
        let (r, c) = (leaf(&tuple.elems[0])?, leaf(&tuple.elems[1])?);
        // The body is exactly one statement.
        let [syn::Stmt::Expr(syn::Expr::Assign(assign), Some(_))] = each.body.stmts.as_slice()
        else {
            return None;
        };
        let (target, c_at) = deref_path(&assign.left)?;
        if target != c {
            return None;
        }
        let counted = method0(uncapped(&assign.right), "available_at_entry")?;
        let queue = fields_place(&counted.receiver)?;
        if queue.name != r {
            return None;
        }
        Some(Self {
            stmt,
            each,
            q,
            k,
            r,
            c,
            path: queue.fields,
            r_at: queue.at,
            c_at,
        })
    }
}

/// `*<name>`: the name and where it stands.
fn deref_path(expr: &syn::Expr) -> Option<(String, Pos)> {
    match expr {
        syn::Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Deref(_)) => {
            path_name(&unary.expr)
        }
        _ => None,
    }
}

/// One reading of a pass loop as a D8b form: queue collection `q`, counters `k`, the queue
/// binding `r` and the counter binding `c` of the pass pattern.
struct Pass<'a, 'f> {
    each: &'f syn::ExprForLoop,
    at: usize,
    q: &'a Place<'f>,
    k: &'a Place<'f>,
    r: &'a str,
    c: &'a str,
}

impl<'f> Pass<'_, 'f> {
    fn holds(&self, ctx: &Ctx<'f>, steps: &[Step<'f>], site: &PopSite) -> bool {
        let Some((block_at, set)) = self.pair(steps) else {
            return false;
        };
        let Some(set_extent) = ctx.tree.extent(Node::Stmt(set.stmt)) else {
            return false;
        };
        if !ctx.region.holds(set_extent.core, set_extent.end) {
            return false;
        }
        let (Some(set_q), Some(set_k)) = (
            ctx.binding_of_path(&set.q.name, set.q.at),
            ctx.binding_of_path(&set.k.name, set.k.at),
        ) else {
            return false;
        };
        let (Some(pass_q), Some(pass_k)) = (
            ctx.binding_of_path(&self.q.name, self.q.at),
            ctx.binding_of_path(&self.k.name, self.k.at),
        ) else {
            return false;
        };
        // Condition 1: the roots are the same bindings in both loops.
        if !set_q.same(&pass_q) || !set_k.same(&pass_k) {
            return false;
        }
        // Condition 2: the set loop's bindings are its own pattern's.
        let own = |name: &str, at: Pos| {
            ctx.binding_of_path(name, at)
                .is_some_and(|binding| binding.is_pattern_of(set.each))
        };
        if !own(&set.r, set.r_at) || !own(&set.c, set.c_at) {
            return false;
        }
        self.body_holds(ctx, steps, site, &set)
            && self.window_holds(ctx, steps, block_at, &set, set_extent.end, &pass_q, &pass_k)
    }

    /// Condition 2's pairing: the last set loop before the pass loop, among the statements of the
    /// blocks that enclose it, that has this form for the same `<Q>` and `<K>`.
    fn pair(&self, steps: &[Step<'f>]) -> Option<(usize, SetLoop<'f>)> {
        (0..self.at).rev().find_map(|index| {
            let Step {
                node: Node::Block(block),
                via: Some(Via::Stmt(before)),
            } = steps[index]
            else {
                return None;
            };
            block.stmts[..before].iter().rev().find_map(|stmt| {
                SetLoop::of(stmt)
                    .filter(|set| set.q.spelled_like(self.q) && set.k.spelled_like(self.k))
                    .map(|set| (index, set))
            })
        })
    }

    /// Condition 5: the guard, the decrement and the pop in the pass body.
    fn body_holds(
        &self,
        ctx: &Ctx<'f>,
        steps: &[Step<'f>],
        site: &PopSite,
        set: &SetLoop<'f>,
    ) -> bool {
        let stmts = &self.each.body.stmts;
        if stmts.len() < 3 || site.kind != PopKind::Method {
            return false;
        }
        let (Some(guard_at), Some(decrement_at)) =
            (guard(&stmts[0], self.c), decrement(&stmts[1], self.c))
        else {
            return false;
        };
        let Some(third) = ctx.tree.extent(Node::Stmt(&stmts[2])) else {
            return false;
        };
        if !(third.full <= site.pos && site.pos < third.end) {
            return false;
        }
        let Node::Expr(pop) = steps[steps.len() - 1].node else {
            return false;
        };
        let Some(call) = method0(pop, "try_pop") else {
            return false;
        };
        let Some(receiver) = Place::of(&call.receiver) else {
            return false;
        };
        if receiver.name != self.r || receiver.fields != set.path {
            return false;
        }
        let body = (
            pos(self.each.body.brace_token.span.open().start()),
            pos(self.each.body.brace_token.span.close().end()),
        );
        // The pop is the only one in the body.
        if ctx
            .pops
            .iter()
            .filter(|other| body.0 <= other.pos && other.pos < body.1)
            .count()
            != 1
        {
            return false;
        }
        // The body names the counter only in the guard and the decrement, and the queue only in
        // the pop's receiver; each name is the pass pattern's.
        let own = |name: &str| {
            ctx.facts
                .uses_in(name, body.0, body.1)
                .iter()
                .filter(|used| {
                    nearest(&ctx.tree.descend(used.pos), name)
                        .is_some_and(|binding| binding.is_pattern_of(self.each))
                })
                .map(|used| used.pos)
                .collect::<Vec<_>>()
        };
        let mut counter = own(self.c);
        counter.sort_unstable();
        let mut expected = vec![guard_at, decrement_at];
        expected.sort_unstable();
        counter == expected && own(self.r) == vec![receiver.at]
    }

    /// Condition 6: every use of a root in the window is one of the closed list's five.
    #[allow(clippy::too_many_arguments)]
    fn window_holds(
        &self,
        ctx: &Ctx<'f>,
        steps: &[Step<'f>],
        block_at: usize,
        set: &SetLoop<'f>,
        from: Pos,
        queue_root: &Binding<'f>,
        counter_root: &Binding<'f>,
    ) -> bool {
        // The form's outermost loop: the first loop below the block that holds the set loop.
        let Some(outer) = (block_at + 1..=self.at).find(|&index| steps[index].node.is_loop())
        else {
            return false;
        };
        let Some(outer_extent) = ctx.tree.extent(steps[outer].node) else {
            return false;
        };
        let Some(function) = steps[..=self.at]
            .iter()
            .rposition(|step| step.node.is_function())
        else {
            return false;
        };
        let window = Window {
            pass: self,
            set,
            queue_root,
            counter_root,
            outer,
            function: steps[function].node.id(),
        };
        let mut names = vec![self.q.name.as_str()];
        if self.k.name != self.q.name {
            names.push(self.k.name.as_str());
        }
        names.into_iter().all(|name| {
            ctx.facts
                .uses_in(name, from, outer_extent.end)
                .iter()
                .all(|used| window.allows(ctx, name, used))
        })
    }
}

/// The guard `if <cond> { continue; }`, where `<cond>` is `*<c> == 0` or a `||` chain with that as
/// an operand: where `<c>` stands in the operand.
fn guard(stmt: &syn::Stmt, counter: &str) -> Option<Pos> {
    let syn::Stmt::Expr(syn::Expr::If(guard), _) = stmt else {
        return None;
    };
    if guard.else_branch.is_some() {
        return None;
    }
    let [syn::Stmt::Expr(syn::Expr::Continue(skip), Some(_))] = guard.then_branch.stmts.as_slice()
    else {
        return None;
    };
    if skip.label.is_some() {
        return None;
    }
    zero_operand(&guard.cond, counter)
}

fn zero_operand(cond: &syn::Expr, counter: &str) -> Option<Pos> {
    let syn::Expr::Binary(binary) = cond else {
        return None;
    };
    match binary.op {
        syn::BinOp::Or(_) => {
            zero_operand(&binary.left, counter).or_else(|| zero_operand(&binary.right, counter))
        }
        syn::BinOp::Eq(_) if is_int(&binary.right, "0") => {
            let (name, at) = deref_path(&binary.left)?;
            (name == counter).then_some(at)
        }
        _ => None,
    }
}

/// The decrement `*<c> -= 1;`: where `<c>` stands.
fn decrement(stmt: &syn::Stmt, counter: &str) -> Option<Pos> {
    let syn::Stmt::Expr(syn::Expr::Binary(binary), Some(_)) = stmt else {
        return None;
    };
    if !matches!(binary.op, syn::BinOp::SubAssign(_)) || !is_int(&binary.right, "1") {
        return None;
    }
    let (name, at) = deref_path(&binary.left)?;
    (name == counter).then_some(at)
}

/// Condition 6's window: from the end of the set loop to the end of the form's outermost loop.
struct Window<'a, 'b, 'f> {
    pass: &'a Pass<'b, 'f>,
    set: &'a SetLoop<'f>,
    queue_root: &'a Binding<'f>,
    counter_root: &'a Binding<'f>,
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
        let is_queue = binding.same(self.queue_root);
        let is_counter = binding.same(self.counter_root);
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
        let q = is_queue.then_some(&self.set.q);
        let k = is_counter.then_some(&self.set.k);
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
        if is_queue && fields == self.set.q.fields && shared_read(&steps, top) {
            return true;
        }
        // Item 4: a read loop over `<K>`.
        if is_counter && fields == self.set.k.fields && self.read_loop(ctx, &steps, top) {
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
        if bare(&node.method) == super::facts::POP {
            self.found = true;
        }
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        if node
            .path
            .segments
            .last()
            .is_some_and(|last| bare(&last.ident) == super::facts::POP)
        {
            self.found = true;
        }
        syn::visit::visit_expr_path(self, node);
    }
}
