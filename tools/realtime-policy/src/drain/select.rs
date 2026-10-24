//! The selecting outer loop (#1418 D1): every loop around a counted drain, inside its region, must
//! drain a different queue on each pass. [`super::outer`] proves that the loop is finite; this
//! module proves that it selects.
//!
//! The walk starts from the counted receiver (`<recv>` of `<recv>.available_at_entry()`) and goes
//! outward over the loops between the counted loop and the region's top. It holds one receiver at a
//! time, a root identifier with `.field`, `[<index>]` and zero-argument `.method()` steps, and
//! resolves the root with the one resolver ([`super::scope`]). A loop selects in one of three ways:
//!
//! - **Pattern root.** The root is bound by this `for` loop's pattern, in the slot of the element
//!   of its collection (`for control in &mut controls`, `for (index, queue) in
//!   self.queues.iter_mut()`, `for (lane, control) in controls.iter_mut().enumerate()`). The
//!   binding is a plain identifier: no `mut`, no `ref`, no sub-pattern. The receiver outward is the
//!   collection (`controls`, `self.queues`). A root bound in the slot of a `.zip(..)` argument
//!   selects this loop only: no loop outside it selects through it.
//! - **Index.** The receiver holds exactly one index by the identifier that takes a new value on
//!   each pass, and that index resolves to this loop's pattern: the lone non-`mut` pattern of a
//!   range loop (`for <i> in 0..<bound>`; whether the range ends is [`super::outer`]'s), or the
//!   index slot of a trailing `.enumerate()` whose collection is not
//!   `&mut` of anything (`(&mut cycle).enumerate()` may never end). `controls[lane % 2]` and
//!   `controls[0]` do not select. The receiver outward is the base before that index (`controls` of
//!   `controls[lane]`, `self.banks[bank].controls` of `self.banks[bank].controls[lane]`).
//! - **Rebinding.** The root is bound inside this loop's body by a `let`, a `let .. else` or an
//!   `if let` (an `else if let` included) whose pattern is a plain identifier or `Some(<ident>)`,
//!   and whose initializer is a receiver of the same grammar, optionally behind one `&` or `&mut`,
//!   optionally followed by one `.as_mut()` or `.as_ref()` (`let control = &mut controls[lane];`,
//!   `let Some(control) = control.as_mut() else { .. };`). The walk then reads the initializer's
//!   receiver in its place, with the steps the original had after the root, and asks the same loop
//!   again. A chain of any depth gives one answer: no cap.
//!
//! A loop that selects in none of these ways is refused, and so is every `while` and `loop`. A
//! loop whose pattern binds nothing (`_`, `()`) never selects (D2). A rebinding under a
//! `cfg`-family attribute is refused: it may not be compiled.
//!
//! **Invariance (R3).** A selecting loop must also keep the queue it selects in place for the whole
//! pass. Every root the walk read in the loop's turn (the receiver's own root, the root of each
//! index place, and each rebinding's) is held, over the loop's body, to the place rule of #1426
//! ([`super::place`]), except at the walk's own reads (the root of a rebinding's initializer and of
//! the collection outward): no assignment to it, no `&mut` of it, no by-value use of a queue root
//! (`mem::swap(control, spare)`), no reorder of the collection it indexes (`controls.swap(0, 1)`),
//! no mention in a closure or in a macro's token tree. Any macro call in a selecting loop's body,
//! other than a trusted `debug_assert!`, is refused too: a `macro_rules!` defined earlier can move
//! the base with no name in the body.
//!
//! A zero-argument call step in a base (`self.pick()[lane]`) is taken at its word, as #1426 R1
//! compares such a receiver by spelling only: the method can return another collection on each
//! pass, which this rule does not read.
//!
//! **Collections are taken at their word (D3).** The walk does not prove that a collection holds
//! each queue once. Iterating `&mut <fields>` yields distinct elements, and an SPSC consumer is not
//! `Clone`, so each element is a different queue.

use syn::visit::{self, Visit};

use super::Ctx;
use super::count::{Counted, under_cfg};
use super::names::bare;
use super::place;
use super::scope::{Binding, BindingKind, nearest};
use super::shapes::{Place, Root, RootKind, Seg, method0, simple_path};
use super::tree::{Node, Pos, Step, is_debug_assert, macro_key, pos};

/// One step of a receiver the walk holds.
#[derive(Clone)]
enum Step2<'f> {
    Field(String),
    Index(&'f syn::Expr),
    Call,
}

/// A receiver the walk holds: a root identifier and the steps after it.
#[derive(Clone)]
struct Chain<'f> {
    root: String,
    root_at: Pos,
    steps: Vec<Step2<'f>>,
}

impl<'f> Chain<'f> {
    /// The receiver `expr` names, in the grammar of [`super::shapes::receiver`].
    fn of(expr: &'f syn::Expr) -> Option<Self> {
        let mut steps = Vec::new();
        let mut at = expr;
        loop {
            match at {
                syn::Expr::Path(path) => {
                    let (root, root_at) = simple_path(path)?;
                    steps.reverse();
                    return Some(Self {
                        root,
                        root_at,
                        steps,
                    });
                }
                syn::Expr::Field(field) => {
                    let syn::Member::Named(member) = &field.member else {
                        return None;
                    };
                    steps.push(Step2::Field(bare(member)));
                    at = &field.base;
                }
                syn::Expr::MethodCall(call) => {
                    if call.turbofish.is_some() || !call.args.is_empty() {
                        return None;
                    }
                    steps.push(Step2::Call);
                    at = &call.receiver;
                }
                syn::Expr::Index(index) => {
                    let literal = matches!(
                        &*index.index,
                        syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Int(_),
                            ..
                        })
                    );
                    if !literal && Place::of(&index.index).is_none() {
                        return None;
                    }
                    steps.push(Step2::Index(&index.index));
                    at = &index.expr;
                }
                _ => return None,
            }
        }
    }

    /// The roots of the receiver for the place rule: its own root, then the root of each index
    /// place.
    fn roots(&self) -> Vec<Root> {
        let path: Vec<Seg> = self
            .steps
            .iter()
            .map_while(|step| match step {
                Step2::Field(name) => Some(Seg::Field(name.clone())),
                Step2::Index(_) => Some(Seg::Index),
                Step2::Call => None,
            })
            .collect();
        let mut roots = vec![Root {
            name: self.root.clone(),
            at: self.root_at,
            kind: RootKind::Queue,
            path,
        }];
        for step in &self.steps {
            if let Step2::Index(index) = step
                && let Some(place) = Place::of(index)
            {
                roots.push(Root {
                    name: place.name,
                    at: place.at,
                    kind: RootKind::Index,
                    path: place.fields.into_iter().map(Seg::Field).collect(),
                });
            }
        }
        roots
    }
}

/// Where a pattern binds the root.
enum Slot<'f> {
    /// The element of the collection `<fields>`: the receiver outward.
    Element(&'f syn::Expr),
    /// A `.zip(..)` argument's element: no loop outside selects through it.
    Zip,
}

/// Whether every loop of `steps[top..at]` selects the queue that the loop at `steps[at]` counts.
pub(super) fn selects<'f>(
    ctx: &Ctx<'f>,
    steps: &[Step<'f>],
    top: usize,
    at: usize,
    counted: &Counted<'f>,
) -> bool {
    let loops: Vec<usize> = (top..at)
        .rev()
        .filter(|&i| steps[i].node.is_loop())
        .collect();
    if loops.is_empty() {
        return true;
    }
    let Some(mut chain) = Chain::of(counted.receiver) else {
        return false;
    };
    // The positions of the walk's own reads, which the place rule does not judge.
    let mut own: Vec<Pos> = Vec::new();
    let mut ended = false;
    for &g in &loops {
        let Node::Expr(syn::Expr::ForLoop(each)) = steps[g].node else {
            return false;
        };
        if ended || calls_a_macro(ctx, each) {
            return false;
        }
        let body = (
            pos(each.body.brace_token.span.open().start()),
            pos(each.body.brace_token.span.close().end()),
        );
        // The roots this loop's turn reads, with their bindings.
        let mut held: Vec<(Root, Binding<'f>)> = Vec::new();
        let next = loop {
            let Some(binding) = resolve(ctx, &chain.root, chain.root_at) else {
                return false;
            };
            for root in chain.roots() {
                let Some(found) = resolve(ctx, &root.name, root.at) else {
                    return false;
                };
                if root.name != "self" {
                    ctx.note_read(&root.name, found);
                }
                held.push((root, found));
            }
            if binding.is_pattern_of(each) {
                match slot(&each.pat, &each.expr, &chain.root) {
                    Some(Slot::Element(collection)) => break Chain::of(collection),
                    Some(Slot::Zip) => {
                        ended = true;
                        break None;
                    }
                    None => return false,
                }
            }
            if let Some(base) = by_index(ctx, each, &chain) {
                break Some(base);
            }
            match rebound(ctx, body, &binding, &chain) {
                Some(inner) => {
                    own.push(inner.root_at);
                    chain = inner;
                }
                None => return false,
            }
        };
        if let Some(next) = &next {
            own.push(next.root_at);
        }
        let kept = held.iter().all(|(root, binding)| {
            place::untouched_except(ctx, root, binding, body.0, body.1, &own)
        });
        if !kept {
            return false;
        }
        match next {
            Some(next) => chain = next,
            None if ended => {}
            None => return false,
        }
    }
    true
}

/// Where the pattern `pat` of a loop over `iter` binds `name`: the element of the collection, or a
/// `.zip(..)` argument's element. `None` when it binds it elsewhere (an `.enumerate()` index), more
/// than once, not as a plain identifier, or when the collection is not `<fields>` behind `&`,
/// `&mut`, `.iter()` or `.iter_mut()`, or a slice parameter.
fn slot<'f>(pat: &syn::Pat, iter: &'f syn::Expr, name: &str) -> Option<Slot<'f>> {
    if let Some(call) = method0(iter, "enumerate") {
        let (index, element) = pair(pat)?;
        if binders(index, name) != 0 {
            return None;
        }
        return slot(element, &call.receiver, name);
    }
    if let Some(call) = super::shapes::method(iter, "zip")
        && call.args.len() == 1
    {
        let (element, argument) = pair(pat)?;
        return match (binders(element, name), binders(argument, name)) {
            (0, 1) if plain_binder(argument, name) => Some(Slot::Zip),
            (_, 0) => slot(element, &call.receiver, name),
            _ => None,
        };
    }
    if binders(pat, name) != 1 || !plain_binder(pat, name) {
        return None;
    }
    let collection = match iter {
        syn::Expr::Reference(reference) => &*reference.expr,
        syn::Expr::MethodCall(_) => {
            let call = method0(iter, "iter").or_else(|| method0(iter, "iter_mut"))?;
            &*call.receiver
        }
        // A collection by value (a slice parameter); whether it is finite is [`super::outer`]'s.
        syn::Expr::Path(_) | syn::Expr::Field(_) => iter,
        _ => return None,
    };
    Place::of(collection).map(|_| Slot::Element(collection))
}

/// The two elements of a two-element tuple pattern.
fn pair(pat: &syn::Pat) -> Option<(&syn::Pat, &syn::Pat)> {
    let syn::Pat::Tuple(tuple) = pat else {
        return None;
    };
    (tuple.elems.len() == 2).then(|| (&tuple.elems[0], &tuple.elems[1]))
}

/// How many times `pat` binds `name`. A macro pattern binds it as often as it may: refuse.
fn binders(pat: &syn::Pat, name: &str) -> usize {
    struct Count<'a> {
        name: &'a str,
        found: usize,
    }
    impl<'ast> Visit<'ast> for Count<'_> {
        fn visit_pat_ident(&mut self, node: &'ast syn::PatIdent) {
            if bare(&node.ident) == self.name {
                self.found += 1;
            }
            visit::visit_pat_ident(self, node);
        }
        fn visit_pat(&mut self, node: &'ast syn::Pat) {
            if matches!(node, syn::Pat::Macro(_) | syn::Pat::Verbatim(_)) {
                self.found += 2;
            }
            visit::visit_pat(self, node);
        }
    }
    let mut count = Count { name, found: 0 };
    count.visit_pat(pat);
    count.found
}

/// Whether every identifier pattern of `name` in `pat` is plain: no `mut`, no `ref`, no
/// sub-pattern.
fn plain_binder(pat: &syn::Pat, name: &str) -> bool {
    struct Plain<'a> {
        name: &'a str,
        plain: bool,
    }
    impl<'ast> Visit<'ast> for Plain<'_> {
        fn visit_pat_ident(&mut self, node: &'ast syn::PatIdent) {
            if bare(&node.ident) == self.name
                && (node.by_ref.is_some() || node.mutability.is_some() || node.subpat.is_some())
            {
                self.plain = false;
            }
            visit::visit_pat_ident(self, node);
        }
    }
    let mut check = Plain { name, plain: true };
    check.visit_pat(pat);
    check.plain
}

/// The base of `chain` when it holds exactly one index by the identifier that the loop `each`
/// gives a new value on each pass, and that index resolves to the loop's pattern.
fn by_index<'f>(ctx: &Ctx<'f>, each: &'f syn::ExprForLoop, chain: &Chain<'f>) -> Option<Chain<'f>> {
    let ident = pass_index(each)?;
    let mut found = None;
    for (at, step) in chain.steps.iter().enumerate() {
        let Step2::Index(index) = step else {
            continue;
        };
        let Some(place) = Place::of(index) else {
            continue;
        };
        if place.name != ident {
            continue;
        }
        if found.is_some() || !place.fields.is_empty() {
            return None;
        }
        let binding = resolve(ctx, &place.name, place.at)?;
        if !binding.is_pattern_of(each) {
            return None;
        }
        found = Some(at);
    }
    let at = found?;
    Some(Chain {
        root: chain.root.clone(),
        root_at: chain.root_at,
        steps: chain.steps[..at].to_vec(),
    })
}

/// The identifier that takes a new value on each pass of `each`: the lone plain pattern of
/// `0..<bound>`, or the index slot of a trailing `.enumerate()` not over `&mut`.
fn pass_index(each: &syn::ExprForLoop) -> Option<String> {
    let ident = |pat: &syn::Pat| match pat {
        syn::Pat::Ident(ident)
            if ident.by_ref.is_none() && ident.mutability.is_none() && ident.subpat.is_none() =>
        {
            Some(bare(&ident.ident))
        }
        _ => None,
    };
    match &*each.expr {
        // Any range yields a new integer on each pass; whether it ends is [`super::outer`]'s.
        syn::Expr::Range(_) => ident(&each.pat),
        expr => {
            let call = method0(expr, "enumerate")?;
            let mut collection = &*call.receiver;
            while let syn::Expr::Paren(inner) = collection {
                collection = &inner.expr;
            }
            if matches!(collection, syn::Expr::Reference(reference) if reference.mutability.is_some())
            {
                return None;
            }
            ident(pair(&each.pat)?.0)
        }
    }
}

/// The receiver that a rebinding of `chain`'s root inside the loop body `body` reads, with the
/// steps that `chain` has after its root.
fn rebound<'f>(
    ctx: &Ctx<'f>,
    body: (Pos, Pos),
    binding: &Binding<'f>,
    chain: &Chain<'f>,
) -> Option<Chain<'f>> {
    let (pat, init, extent) = match binding.kind {
        BindingKind::Let { stmt, local } => {
            if under_cfg(local) {
                return None;
            }
            (
                &local.pat,
                &*local.init.as_ref()?.expr,
                ctx.tree.extent(Node::Stmt(stmt))?,
            )
        }
        BindingKind::Pattern {
            owner: owner @ Node::Expr(syn::Expr::If(branch)),
        } => {
            let syn::Expr::Let(condition) = &*branch.cond else {
                return None;
            };
            (&*condition.pat, &*condition.expr, ctx.tree.extent(owner)?)
        }
        _ => return None,
    };
    if !(body.0 <= extent.full && extent.end <= body.1) || !binds_alone(pat, &chain.root) {
        return None;
    }
    let mut source = init;
    if let Some(call) = method0(source, "as_mut").or_else(|| method0(source, "as_ref")) {
        source = &call.receiver;
    }
    if let syn::Expr::Reference(reference) = source {
        source = &reference.expr;
    }
    let mut inner = Chain::of(source)?;
    inner.steps.extend(chain.steps.iter().cloned());
    Some(inner)
}

/// Whether `pat` is `<name>` or `Some(<name>)`, the identifier plain.
fn binds_alone(pat: &syn::Pat, name: &str) -> bool {
    let plain = |pat: &syn::Pat| {
        matches!(pat, syn::Pat::Ident(ident)
            if ident.by_ref.is_none()
                && ident.mutability.is_none()
                && ident.subpat.is_none()
                && bare(&ident.ident) == name)
    };
    match pat {
        syn::Pat::TupleStruct(some) => {
            some.qself.is_none()
                && some.path.leading_colon.is_none()
                && some.path.segments.len() == 1
                && some.path.segments[0].arguments.is_none()
                && bare(&some.path.segments[0].ident) == "Some"
                && some.elems.len() == 1
                && plain(&some.elems[0])
        }
        other => plain(other),
    }
}

/// Whether the body of `each` calls a macro other than a trusted `debug_assert!`.
fn calls_a_macro(ctx: &Ctx<'_>, each: &syn::ExprForLoop) -> bool {
    struct Calls<'a, 'c> {
        ctx: &'a Ctx<'c>,
        found: bool,
    }
    impl<'ast> Visit<'ast> for Calls<'_, '_> {
        fn visit_macro(&mut self, node: &'ast syn::Macro) {
            if !(is_debug_assert(node) && self.ctx.facts.debug_args.contains_key(&macro_key(node)))
            {
                self.found = true;
            }
        }
    }
    let mut calls = Calls { ctx, found: false };
    calls.visit_block(&each.body);
    calls.found
}

/// The binding that the path `name` at `at` resolves to. A use in the initializer of an `if let`
/// resolves outside it: the resolver reads an `if let` as binding in all of its condition, which
/// only refuses more, but the walk reads that initializer as the receiver the `if let` rebinds.
fn resolve<'f>(ctx: &Ctx<'f>, name: &str, at: Pos) -> Option<Binding<'f>> {
    let mut found = ctx.binding_of_path(name, at)?;
    let steps = ctx.tree.descend(at);
    loop {
        let BindingKind::Pattern {
            owner: owner @ Node::Expr(syn::Expr::If(branch)),
        } = found.kind
        else {
            return Some(found);
        };
        let syn::Expr::Let(condition) = &*branch.cond else {
            return Some(found);
        };
        let init = ctx.tree.extent(Node::Expr(&condition.expr))?;
        if !(init.full <= at && at < init.end) {
            return Some(found);
        }
        let cut = steps.iter().position(|step| step.node.id() == owner.id())?;
        found = nearest(&steps[..cut], name)?;
    }
}
