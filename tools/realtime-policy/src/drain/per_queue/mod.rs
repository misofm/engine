//! The per-queue counted pop (D8b): one remaining count per queue, read at entry, that each pop
//! is guarded by and decrements.
//!
//! The form is a set loop that reads every queue's count at entry into a counter, and a pass loop
//! that skips a queue whose counter is zero, decrements it, and pops that queue. Six conditions
//! make it bound every queue: the names resolve to the same bindings in both loops (1), the set
//! loop only reads the counts (2), the loops are the form's own (3), the pass loop pairs queue i
//! with counter i (4), its body is the guard, the decrement and one pop of the counted queue (5),
//! and nothing in the window between the loops writes the counters or replaces the queues (6).

mod window;

use super::Ctx;
use super::facts::{PopKind, PopSite};
use super::names::bare;
use super::scope::{Binding, nearest};
use super::shapes::{Place, is_int, method, method0, path_name, uncapped};
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

/// Where a pass loop's set loop sits, and what the window of condition 6 reads.
struct Located<'a, 'f> {
    steps: &'a [Step<'f>],
    /// The step of the block that holds the set loop.
    block_at: usize,
    set: &'a SetLoop<'f>,
    /// Where the set loop ends: the window starts there.
    set_end: Pos,
    queue_root: &'a Binding<'f>,
    counter_root: &'a Binding<'f>,
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
            && window::holds(
                self,
                ctx,
                &Located {
                    steps,
                    block_at,
                    set: &set,
                    set_end: set_extent.end,
                    queue_root: &pass_q,
                    counter_root: &pass_k,
                },
            )
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
