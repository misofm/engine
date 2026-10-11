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
//!   under a `cfg`-family attribute, and a `static`, `const` or `use` of the region that can name
//!   the count, are refused. **Constructors (D3)** are in [`items`]: `from_fn`, `repeat_with` and
//!   `successors` are refused anywhere in a region, except `core::array::from_fn` and
//!   `std::array::from_fn`, whose closure is a finite outer loop and never bounds a pop of its own.
//!
//! What stays outside the rule (D4): recursion; a pop in a helper function (`begin_block` calling
//! `drain_controls` is one); a loop in a caller, outside the region (a loop that encloses the
//! region inside the same function is refused above, not a limit); and a macro, defined elsewhere,
//! that expands to a loop. A pop in a closure or a macro is judged by the loops around it as if
//! the closure or macro were not there; a later slice refuses it.
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

use std::collections::BTreeSet;

use crate::boundary::{Boundaries, RegionNodes};
use crate::leaf::Leaf;
use crate::markers::Region;
use crate::report::{Class, Finding};
use crate::source::Source;
use facts::{Facts, PopSite, Use};
use scope::{Binding, nearest};
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

    /// Whether a `static`, `const` or `use` of the pop's region can name `name` (D2).
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
        let facts = Facts::of(file);
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
    let steps = ctx.tree.descend(site.pos);
    if !reached(&steps, site) {
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
    count::bounds(ctx, &steps, at) || per_queue::bounds(ctx, &steps, at, site)
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
