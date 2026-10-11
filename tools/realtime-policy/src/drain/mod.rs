//! The drain-bound rule: every render-thread `try_pop` runs at most once per record counted at
//! block entry by `available_at_entry`, per pass of the finite loops around its drain.
//!
//! A loop that pops until the queue is empty keeps popping whatever a concurrent producer
//! publishes while it runs: data-dependent, unbounded render work. The rule reads each file's
//! syntax tree, so layout, comments and literals play no part, and judges every pop that stands
//! between a region's markers:
//!
//! - **Mapping (D1).** A region's nodes are the maximal nodes between its markers
//!   ([`crate::boundary::RegionNodes`]); a region with a marker in a macro token tree has no
//!   syntax tree, and any pop in it is refused. Every node the rule reads must lie in the pop's
//!   region, and a loop that encloses the region is refused.
//! - **Scope (D2).** One resolver ([`scope`]) finds the nearest binding of a name.
//! - **Visited syntax (D4).** Only the nodes of [`tree`]'s allow-list are walked. A pop below any
//!   other node is not reached, and an unreached pop is refused.
//! - **Pops (D5), counts (D6), reads (D7) and the bounding loop (D8)** are in [`facts`], [`count`]
//!   and [`shapes`]; D8b's per-queue form is [`per_queue`].
//! - **Outer loops (D1)** are in [`outer`]: every other loop around a pop, inside its region,
//!   must be finite. **Attributes and items (D2, D9)** are in [`count`], [`outer`], [`per_queue`]
//!   and [`items`]: a count binding under a `cfg`-family attribute, and an item of any kind of the
//!   region (`const`, `static`, `struct`, `enum`, `union`, `fn`, `type`, `trait`, `mod`, or a `use`)
//!   that declares the name of any binding a rule read to bound the pop, are refused. The reads are
//!   the count, its alias source, a slice parameter, the root of an outer loop's collection
//!   (`controls` of `&controls`, `controls.iter_mut()`, `cfg` of `&cfg.controls`) and the roots
//!   `<Q>` and `<K>` of a per-queue form. **Constructors (D3)** are in [`items`]: `from_fn`,
//!   `repeat_with` and `successors` are refused anywhere in a region, except the token of
//!   `core::array::from_fn` and `std::array::from_fn`. A pop in the closure of either is refused
//!   like a pop in any closure (B2a-D1).
//! - **Macros and attributes that can declare an item (D10).** An expansion is invisible to the
//!   item scan, and an item declared in a block shadows a binding of an enclosing block, or a
//!   function's parameter, for the whole block it lands in. So the drain is refused when it holds
//!   a macro invocation in statement or item position (`name!(..);`, `name![..];`, `name! {..}`)
//!   or an item with an attribute other than `cfg`, a lint level or `doc` (a derive and an
//!   attribute macro emit items beside their item, and `cfg_attr` can expand to a derive) inside
//!   the span of a binding the rules read. For a `let` (the count, its alias source, or a
//!   collection root) the span runs from the `let` to the end of the statement, in the `let`'s own
//!   block, that holds the pop: it runs past the loop, so an item declared after the loop in an
//!   inner block reaches its header too. An item in the `let`'s own block cannot shadow it: the
//!   `let` becomes refutable and does not compile for a `const` or a unit struct, and it wins over a
//!   `fn`. For a parameter (a slice parameter, a collection root, a queue root) the span is the
//!   whole `fn` body, because a parameter's block is the body, and an item anywhere in it, after
//!   the loop included, shadows the parameter. For a pattern (the `for` that binds a collection
//!   root) the span is the node that binds it. A `debug_assert!` is exempt while it is trusted
//!   ([`trusted`]); a macro in expression position cannot declare an item and is not read.
//!   `debug_assert_eq!` and `debug_assert_ne!` are not exempt: an accepted over-refusal (widening
//!   the exemption needs a root ruling).
//! - **Region ends (B1b-D9)** are in [`crate::boundary`]: a region ends in the container of its
//!   begin marker or in one that encloses it, so no block continues past the end marker with the
//!   rest of its items unread.
//!
//! Pops that run where the surrounding loops cannot bound them are refused, wherever they stand
//! (B2a):
//!
//! - **Closures (D1).** A pop in a closure is refused, the closure of `array::from_fn` included:
//!   it runs wherever the closure is called, and a closure that a loop calls, or that a repeating
//!   adapter holds (`repeat(()).map_while(..)`), pops until the queue is empty. The path roots
//!   `core` and `std` of `core::array::from_fn` can be redirected (a workspace `mod`, a `use .. as`,
//!   a macro-made module, a Cargo dependency key), so that closure is no finite outer loop either.
//! - **Macros (D2).** A `try_pop` identifier in any macro's token tree is refused: in a macro
//!   call, a `macro_rules!` body and the arguments of `debug_assert!`. A count named inside
//!   `debug_assert!` is a read only while no file under the scan roots gives `debug_assert` another
//!   meaning ([`trusted`]): a `debug_assert` identifier that a `!` does not directly follow is a
//!   shadow, and then `debug_assert!` is any other macro, and a drain that names its count in one
//!   is refused in every file.
//! - **Function values (D3).** A path expression whose last segment is `try_pop` is allowed only as
//!   the callee of a call (`Consumer::try_pop(control)`) or as the only argument of a `.map(..)`
//!   method call whose receiver is `<fields>.as_mut()` (`self.control.as_mut().map(Consumer::try_pop)`):
//!   `Option::map` runs it at most once, and the loops around it bound it. On any other receiver
//!   `.map` may be an iterator adapter that pops once per item with no count. Any other use
//!   (`let pop = Consumer::try_pop;`, an argument, a field) is refused.
//! - **Trusted names (D2, D3).** The rule trusts `map`, `as_mut`, `try_pop` and `debug_assert`
//!   to mean what they mean in the standard library and in `Consumer`. [`trusted`] reads every
//!   `.rs` file under the roots and refuses a definition, import, rename or macro-emitted use of
//!   one of them in a form that the rule does not allow, and any `proc_macro` identifier.
//!
//! What stays outside the rule (D4): recursion; a pop in a helper function (`begin_block` calling
//! `drain_controls` is one); a loop in a caller, outside the region (a loop that encloses the
//! region inside the same function is refused above, not a limit); and a macro, defined elsewhere,
//! that expands to a loop.
//!
//! The limits of the tool's threat model (#1438 Hazards): code outside the workspace source, which
//! the tool cannot read, and type-level behaviour, because the rule reads syntax and not types.
//! The tool judges the workspace's own source, so:
//!
//! - **An iterator that never ends, borrowed from a field or a local (type-level).** `for _ in
//!   &mut rounds` and `rounds.iter_mut()` are finite outer loops by their form (D1), so `let mut
//!   rounds = 0_u32..; for _ in &mut rounds { <counted drain> }` passes, and runs without end. The
//!   `main` gate documented the same hole. Restructure so that no unbounded iterator is reachable
//!   by that name; a ruling is needed to refuse these forms. For the same reason a field whose
//!   type has an `as_mut` or a `map` of its own that loops is outside the check when the type is
//!   not defined in the workspace.
//! - **A registry crate (code outside the workspace).** One that exports a trait or type method
//!   named `map` or `as_mut`, or a `debug_assert` macro imported with `#[macro_use] extern crate`
//!   or a glob `use`, or a proc macro, is outside the scan of [`trusted`]. No crate of the
//!   workspace does this.
//! - **Auto-deref and impl resolution (type-level).** Which `Deref` target or which impl a method
//!   call resolves to is not read.
//!
//! It refuses these bounded drains (D7; restructure the drain to pass): a count used other than as
//! the left operand of a comparison before its loop (`0 == available`, `apply(available)`,
//! `available as u32`); a count capped other than by `.min(..)` (`cmp::min`, `.clamp(..)`); an outer
//! range other than `0..<bound>` (`1..n`, `0..=n`, `.rev()`, `.take(n)`); a bare local iterated by
//! value; and a pop once per pass of a collection loop, with no count.

mod count;
mod facts;
mod items;
mod names;
mod outer;
mod per_queue;
mod scope;
mod shapes;
mod tree;
mod trusted;

use std::cell::RefCell;
use std::collections::BTreeSet;

use crate::boundary::{Boundaries, RegionNodes};
use crate::leaf::Leaf;
use crate::markers::Region;
use crate::report::{Class, Finding};
use crate::source::Source;
use facts::{Facts, PopKind, PopSite, Use};
use scope::{Binding, BindingKind, nearest};
use tree::{Node, Pos, Step, Tree, Via};
pub(crate) use trusted::NameScan;

/// What a judgment reads: the file's tree, its facts, the pop's region, and every pop of the file.
struct Ctx<'f> {
    boundaries: &'f Boundaries,
    tree: &'f Tree<'f>,
    facts: &'f Facts,
    region: &'f RegionNodes,
    /// The leaves between the pop's region's markers.
    leaves: &'f [Leaf],
    pops: &'f [PopSite],
    /// The names, with their bindings, that the rules judged so far read to bound the pop: the
    /// count, its alias source, a slice parameter, the root of an outer loop's collection, the
    /// roots of a per-queue form (D9, D10). A rule that fails takes its entries back
    /// ([`Ctx::rewind`]).
    reads: RefCell<Vec<(String, Binding<'f>)>>,
}

impl<'f> Ctx<'f> {
    /// The binding that the single-segment path expression named `name` at `at` resolves to.
    /// `None` when there is no such path there.
    fn binding_of_path(&self, name: &str, at: Pos) -> Option<Binding<'f>> {
        let steps = self.tree.descend(at);
        match steps.last()?.node {
            Node::Expr(syn::Expr::Path(path))
                if shapes::simple_path(path)
                    .is_some_and(|(found, place)| found == name && place == at) => {}
            _ => return None,
        }
        nearest(&steps, name)
    }

    /// Note that a rule read `binding`, which `name` resolved to, to bound the pop (D9, D10).
    fn note_read(&self, name: &str, binding: Binding<'f>) {
        self.reads.borrow_mut().push((name.to_owned(), binding));
    }

    /// How many reads are noted, to hand to [`Ctx::rewind`].
    fn mark(&self) -> usize {
        self.reads.borrow().len()
    }

    /// Forget the reads noted after `mark`: the rule that noted them did not hold.
    fn rewind(&self, mark: usize) {
        self.reads.borrow_mut().truncate(mark);
    }

    /// Whether an item of the pop's region can name `name` (D2, D9).
    fn names_item(&self, name: &str) -> bool {
        items::names_item(self.leaves, name)
    }

    /// The uses of `name` in `from..to` that resolve to `binding`. A use that sits where the walk
    /// cannot reach resolves as far as it can be placed.
    fn resolved_uses(&self, name: &str, from: Pos, to: Pos, binding: &Binding<'f>) -> Vec<Use> {
        self.facts
            .uses_in(name, from, to)
            .iter()
            .copied()
            .filter(|used| {
                nearest(&self.tree.descend(used.pos), name).is_some_and(|found| found.same(binding))
            })
            .collect()
    }
}

/// Check the pops between the markers of `regions`, the regions of `source`, which parsed as `file`.
pub(crate) fn check(
    source: &Source,
    file: &syn::File,
    boundaries: &Boundaries,
    regions: &[Region],
    trust_debug_assert: bool,
    findings: &mut Vec<Finding>,
) {
    let mut refused = BTreeSet::new();
    // D3: a constructor that loops with no loop keyword is refused anywhere in a region, whether or
    // not a pop stands in it.
    for region in regions {
        let leaves = boundaries.leaves_between(
            Boundaries::marker_start(region.begin),
            Boundaries::marker_start(region.end),
        );
        refused.extend(items::constructor_lines(leaves));
    }
    let mut sites = facts::pops(file);
    sites.sort_by_key(|site| site.pos);
    // The region number of each pop that stands between markers.
    let placed: Vec<(usize, PopSite)> = sites
        .iter()
        .filter_map(|site| {
            regions
                .iter()
                .position(|region| region.begin < site.pos.0 && site.pos.0 < region.end)
                .map(|index| (index, *site))
        })
        .collect();
    if !placed.is_empty() {
        let facts = Facts::of(file, trust_debug_assert);
        let tree = Tree::new(file, boundaries, &facts.debug_args);
        let nodes: Vec<RegionNodes> = regions
            .iter()
            .enumerate()
            .map(|(index, region)| boundaries.region_nodes(index, region))
            .collect();
        for (index, site) in placed {
            let (from, to) = nodes[index].bounds();
            let ctx = Ctx {
                boundaries,
                tree: &tree,
                facts: &facts,
                region: &nodes[index],
                leaves: boundaries.leaves_between(from, to),
                pops: &sites,
                reads: RefCell::new(Vec::new()),
            };
            if !bounded(&ctx, &site) {
                refused.insert(site.pos.0);
            }
        }
    }
    findings.extend(refused.into_iter().map(|line| {
        Finding::at_line(
            Class::UnboundedTryPop,
            &source.path,
            line,
            source.line_text(line),
        )
    }));
}

/// Whether the pop is bounded (D1, D4, D8, D8b).
fn bounded<'f>(ctx: &Ctx<'f>, site: &PopSite) -> bool {
    // A `try_pop` token in a macro's token tree runs wherever the macro expands: no loop around
    // the call bounds it (B2a-D2).
    if site.kind == PopKind::MacroToken {
        return false;
    }
    let steps = ctx.tree.descend(site.pos);
    if !reached(&steps, site) {
        return false;
    }
    // A pop in a closure runs wherever the closure is called, and a function value is called
    // wherever it is passed (B2a-D1, D3).
    if in_closure(&steps) || (site.kind == PopKind::Path && !called_in_place(&steps)) {
        return false;
    }
    // The outermost node of the region on the path.
    let Some(top) = steps.iter().position(|step| is_region_node(ctx, step)) else {
        return false;
    };
    // A loop that encloses the region is not a bounded outer loop.
    if (0..top).any(|at| steps[at].node.is_loop()) {
        return false;
    }
    // A pop in a loop header runs once per iteration.
    if steps
        .iter()
        .any(|step| step.node.is_loop() && step.via == Some(Via::Header))
    {
        return false;
    }
    // The innermost loop whose body holds the pop.
    let Some(at) = (0..steps.len())
        .rev()
        .find(|&at| steps[at].node.is_loop() && steps[at].via == Some(Via::Body))
    else {
        return true;
    };
    // D1: every other loop of the region around the pop is finite.
    if !(top..at).all(|outer| !steps[outer].node.is_loop() || outer::finite(ctx, &steps, outer)) {
        return false;
    }
    (count::bounds(ctx, &steps, at) || per_queue::bounds(ctx, &steps, at, site))
        && !shadowed_by_an_item(ctx, &steps)
}

/// D9, D10: whether an item, a macro or an attribute can rebind a name that the rules read.
///
/// - **D9.** An item of the region of any kind (`const`, `static`, `struct`, `enum`, `union`, `fn`,
///   `type`, `trait`, `mod`) that declares the name of a binding the rules read, or a `use` that can
///   name it, shadows that binding for the whole block it lands in, wherever it stands. The reads
///   are the count, its alias source, a slice parameter, the root of an outer loop's collection and
///   the roots of a per-queue form.
/// - **D10.** A macro in statement or item position, or an item with an attribute that can emit
///   items, within the span of a read binding can declare such an item (see [`read_span`] for the
///   span of a `let`, a parameter and a pattern).
fn shadowed_by_an_item<'f>(ctx: &Ctx<'f>, steps: &[Step<'f>]) -> bool {
    let reads = ctx.reads.borrow();
    let mut span: Option<(Pos, Pos)> = None;
    for (name, binding) in reads.iter() {
        if ctx.names_item(name) {
            return true;
        }
        let Some((from, to)) = read_span(ctx, steps, binding) else {
            // A binding that cannot be placed is refused: only a fail-closed answer is safe.
            return true;
        };
        span = Some(span.map_or((from, to), |(first, last)| (first.min(from), last.max(to))));
    }
    span.is_some_and(|(from, to)| ctx.facts.declares_items_in(from, to))
}

/// The span of one read binding (D10).
///
/// - A `let`: from the `let` to the end of the statement of its block that holds the pop. An item
///   declared in an inner block after the loop shadows the `let` at the loop's header, so the span
///   runs to the end of that statement. An item in the `let`'s own block cannot shadow it.
/// - A parameter: the whole `fn` body. A parameter's block is the body, and an item anywhere in the
///   body shadows the parameter: a `const` or a macro that declares one, after the loop, at the
///   body's level included.
/// - A pattern (a `for`, `if let`, `while let`, `match` arm or closure): the whole node that binds
///   it.
fn read_span<'f>(ctx: &Ctx<'f>, steps: &[Step<'f>], binding: &Binding<'f>) -> Option<(Pos, Pos)> {
    match binding.kind {
        BindingKind::Let { stmt, .. } => {
            let at = steps.iter().position(|step| {
                matches!((step.node, step.via), (Node::Block(block), Some(Via::Stmt(index)))
                    if block.stmts[..index].iter().any(|before| std::ptr::eq(before, stmt)))
            })?;
            let from = ctx.tree.extent(Node::Stmt(stmt))?.full;
            let to = match (steps[at].node, steps[at].via) {
                (Node::Block(body), Some(Via::Stmt(index))) => {
                    ctx.tree.extent(Node::Stmt(&body.stmts[index]))?.end
                }
                (node, _) => ctx.tree.extent(node)?.end,
            };
            Some((from, to))
        }
        BindingKind::Param { .. } => {
            let function = steps.iter().rposition(|step| step.node.is_function())?;
            let body = ctx.tree.extent(steps.get(function + 1)?.node)?;
            Some((body.full, body.end))
        }
        BindingKind::Pattern { owner } => {
            let extent = ctx.tree.extent(owner)?;
            Some((extent.full, extent.end))
        }
    }
}

/// Whether a closure holds the last node of the path (B2a-D1). A closure runs wherever it is
/// called, so no loop around it bounds a pop in it; the closure of `array::from_fn` is no
/// exception (root Amendment 3, MAJOR-1).
fn in_closure(steps: &[Step<'_>]) -> bool {
    steps
        .iter()
        .any(|step| matches!(step.node, Node::Expr(syn::Expr::Closure(_))))
}

/// Whether the path expression that ends `steps` stands where it is called at once (B2a-D3): as
/// the callee of a call, or as the only argument of `.map(..)` on `<fields>.as_mut()`.
fn called_in_place(steps: &[Step<'_>]) -> bool {
    let Some(parent) = steps.len().checked_sub(2).map(|at| steps[at]) else {
        return false;
    };
    match (parent.node, parent.via) {
        (Node::Expr(syn::Expr::Call(_)), Some(Via::Operand(0))) => true,
        (Node::Expr(syn::Expr::MethodCall(call)), Some(Via::Operand(1))) => {
            names::bare(&call.method) == "map"
                && call.turbofish.is_none()
                && call.args.len() == 1
                && shapes::method0(&call.receiver, "as_mut")
                    .is_some_and(|inner| shapes::Place::of(&inner.receiver).is_some())
        }
        _ => false,
    }
}

/// Whether the last node of the path is the pop itself, or the macro call that holds it.
fn reached(steps: &[Step<'_>], site: &PopSite) -> bool {
    let Some(last) = steps.last() else {
        return false;
    };
    match last.node {
        Node::Expr(syn::Expr::MethodCall(call)) => {
            tree::pos(call.method.span().start()) == site.pos
        }
        Node::Expr(syn::Expr::Path(path)) => path
            .path
            .segments
            .last()
            .is_some_and(|segment| tree::pos(segment.ident.span().start()) == site.pos),
        node => node.macro_call().is_some(),
    }
}

/// Whether the step is a maximal node of the region (D1).
fn is_region_node(ctx: &Ctx<'_>, step: &Step<'_>) -> bool {
    matches!(
        step.node,
        Node::Item(_) | Node::ImplFn(_) | Node::TraitFn(_) | Node::Stmt(_)
    ) && ctx
        .tree
        .extent(step.node)
        .is_some_and(|extent| ctx.region.is_node(extent.core, extent.end))
}
