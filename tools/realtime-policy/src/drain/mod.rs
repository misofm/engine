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
//!   must be finite. **Attributes and items (D2)** are in [`count`] and [`items`]: a count binding
//!   under a `cfg`-family attribute, and an item of any kind of the region (`const`, `static`,
//!   `struct`, `enum`, `union`, `fn`, `type`, `trait`, `mod`, or a `use`) that can name the count,
//!   its alias source or the slice parameter, are refused. **Constructors (D3)** are in [`items`]:
//!   `from_fn`, `repeat_with` and `successors` are refused anywhere in a region, except
//!   `core::array::from_fn` and `std::array::from_fn`, whose closure is a finite outer loop and
//!   never bounds a pop of its own.
//! - **Macros and attributes that can declare an item (D10).** An expansion is invisible to the
//!   item scan, and an item declared in a block shadows a binding of an enclosing block, or a
//!   function's parameter, for the whole block it lands in. So the drain is refused when it holds
//!   a macro invocation in statement or item position (`name!(..);`, `name![..];`, `name! {..}`)
//!   or an item with an attribute other than `cfg`, a lint level or `doc` (a derive and an
//!   attribute macro emit items beside their item, and `cfg_attr` can expand to a derive) inside
//!   the span of a binding the rules read. For a `let` (the count, or its alias source) the span
//!   runs from the `let` to the end of the statement, in the `let`'s own block, that holds the
//!   pop: it runs past the loop, so an item declared after the loop in an inner block reaches its
//!   header too. An item in the `let`'s own block cannot shadow it: the `let` becomes refutable and
//!   does not compile for a `const` or a unit struct, and it wins over a `fn`. For a slice
//!   parameter the span is the whole `fn` body, because a parameter's block is the body, and an
//!   item anywhere in it, after the loop included, shadows the parameter. A `debug_assert!` is
//!   exempt while it is trusted ([`shadow`]); a macro in expression position cannot declare an
//!   item and is not read. `debug_assert_eq!` and `debug_assert_ne!` are not exempt: an accepted
//!   over-refusal (widening the exemption needs a root ruling).
//! - **Region ends (B1b-D9)** are in [`crate::boundary`]: a region ends in the container of its
//!   begin marker or in one that encloses it, so no block continues past the end marker with the
//!   rest of its items unread.
//!
//! Pops that run where the surrounding loops cannot bound them are refused, wherever they stand
//! (B2a):
//!
//! - **Closures (D1).** A pop in a closure is refused: it runs wherever the closure is called, and
//!   a closure that a loop calls, or that a repeating adapter holds (`repeat(()).map_while(..)`),
//!   pops until the queue is empty. The closure of `array::from_fn` is the one exception, a finite
//!   outer loop that never bounds a pop of its own.
//! - **Macros (D2).** A `try_pop` identifier in any macro's token tree is refused: in a macro
//!   call, a `macro_rules!` body and the arguments of `debug_assert!`. A count named inside
//!   `debug_assert!` is a read only while no file under the scan roots defines or imports a macro
//!   of that name ([`shadow`]); then `debug_assert!` is any other macro, and a drain that names its
//!   count in one is refused in every file. Stated limit: a registry crate that exports a
//!   `debug_assert` macro, imported with `#[macro_use] extern crate` or a glob `use`, is outside
//!   the check; no crate of the workspace does this.
//! - **Function values (D3).** A path expression whose last segment is `try_pop` is allowed only as
//!   the callee of a call (`Consumer::try_pop(control)`) or as the only argument of a `.map(..)`
//!   method call (`self.control.as_mut().map(Consumer::try_pop)`), where it is called at once and
//!   the loops around it bound it. Any other use (`let pop = Consumer::try_pop;`, an argument, a
//!   field) is refused.
//!
//! What stays outside the rule (D4): recursion; a pop in a helper function (`begin_block` calling
//! `drain_controls` is one); a loop in a caller, outside the region (a loop that encloses the
//! region inside the same function is refused above, not a limit); and a macro, defined elsewhere,
//! that expands to a loop.
//!
//! Known false passes, because the rule reads syntax and not types:
//!
//! - **An iterator that never ends, borrowed from a field or a local.** `for _ in &mut rounds`
//!   and `rounds.iter_mut()` are finite outer loops by their form (D1), so
//!   `let mut rounds = 0_u32..; for _ in &mut rounds { <counted drain> }` passes, and runs without
//!   end. The `main` gate documented the same hole. Restructure so that no unbounded iterator is
//!   reachable by that name; a ruling is needed to refuse these forms.
//! - **A collection root that an item shadows.** D2's item scan covers the count, its alias source
//!   and the slice parameter, not the root of `&<fields>` or `<fields>.iter()`: a unit struct
//!   named like the root, with an `iter_mut` that repeats, redirects the loop.
//! - **A registry crate** that exports a `debug_assert` macro (see B2a-D2 above).
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
mod shadow;
mod shapes;
mod tree;

use std::cell::RefCell;
use std::collections::BTreeSet;

use crate::boundary::{Boundaries, RegionNodes};
use crate::leaf::Leaf;
use crate::markers::Region;
use crate::report::{Class, Finding};
use crate::source::Source;
use facts::{Facts, PopKind, PopSite, Use};
use scope::{Binding, BindingKind, nearest};
pub(crate) use shadow::debug_assert_shadowed;
use tree::{Node, Pos, Step, Tree, Via, is_loop_at};

/// What a judgment reads: the file's tree, its facts, the pop's region, and every pop of the file.
struct Ctx<'f> {
    boundaries: &'f Boundaries,
    tree: &'f Tree<'f>,
    facts: &'f Facts,
    region: &'f RegionNodes,
    /// The leaves between the pop's region's markers.
    leaves: &'f [Leaf],
    pops: &'f [PopSite],
    /// The bindings that the rules judged so far read to bound the pop: the count, its alias
    /// source, a slice parameter (D10). A rule that fails takes its entries back ([`Ctx::rewind`]).
    reads: RefCell<Vec<Binding<'f>>>,
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

    /// Note that a rule read `binding` to bound the pop (D10).
    fn note_read(&self, binding: Binding<'f>) {
        self.reads.borrow_mut().push(binding);
    }

    /// How many reads are noted, to hand to [`Ctx::rewind`].
    fn mark(&self) -> usize {
        self.reads.borrow().len()
    }

    /// Forget the reads noted after `mark`: the rule that noted them did not hold.
    fn rewind(&self, mark: usize) {
        self.reads.borrow_mut().truncate(mark);
    }

    /// Whether an item of the pop's region can name `name` (D2).
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
    if (0..top).any(|at| is_loop_at(&steps, at)) {
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
        .find(|&at| is_loop_at(&steps, at) && steps[at].via == Some(Via::Body))
    else {
        return true;
    };
    // D1: every other loop of the region around the pop is finite.
    if !(top..at).all(|outer| !is_loop_at(&steps, outer) || outer::finite(ctx, &steps, outer)) {
        return false;
    }
    (count::bounds(ctx, &steps, at) || per_queue::bounds(ctx, &steps, at, site))
        && !declares_an_item(ctx, &steps)
}

/// D10: whether a macro or an attribute within the span of a binding the rules read can declare
/// an item. Such an item shadows a binding for the whole block it lands in, wherever in the block
/// it stands (see [`read_span`] for the span of a `let` and of a parameter).
fn declares_an_item<'f>(ctx: &Ctx<'f>, steps: &[Step<'f>]) -> bool {
    let reads = ctx.reads.borrow();
    let mut span: Option<(Pos, Pos)> = None;
    for binding in reads.iter() {
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
        BindingKind::Pattern => None,
    }
}

/// Whether a closure other than the closure of `array::from_fn` holds the last node of the path
/// (B2a-D1). That closure is a finite outer loop (B1b-D1), judged with the loops around it.
fn in_closure(steps: &[Step<'_>]) -> bool {
    (0..steps.len()).any(|at| {
        matches!(steps[at].node, Node::Expr(syn::Expr::Closure(_)))
            && tree::array_from_fn_call(steps, at).is_none()
    })
}

/// Whether the path expression that ends `steps` stands where it is called at once (B2a-D3): as
/// the callee of a call, or as the only argument of a `.map(..)` method call.
fn called_in_place(steps: &[Step<'_>]) -> bool {
    let Some(parent) = steps.len().checked_sub(2).map(|at| steps[at]) else {
        return false;
    };
    match (parent.node, parent.via) {
        (Node::Expr(syn::Expr::Call(_)), Some(Via::Operand(0))) => true,
        (Node::Expr(syn::Expr::MethodCall(call)), Some(Via::Operand(1))) => {
            names::bare(&call.method) == "map" && call.args.len() == 1
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
