//! The drain-bound rule: every render-thread `try_pop` runs at most once per record counted at
//! block entry by `available_at_entry`. Each drain pops at most the records its queue held when its
//! count was read, and reads a given queue at most once per pass of its outer loops: every loop
//! around the drain is finite and drains a different queue on each pass (#1418 D6). A sequential
//! double drain of one queue stays outside the rule.
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
//! - **The counted queue (#1426 D1, D2).** A pop that a counted loop bounds pops the receiver the
//!   loop counts: it is spelled `<recv>.try_pop()` (any other spelling is refused), `<recv>` is
//!   the counted receiver token for token, each of its roots resolves to the count's binding, and
//!   no written replacement of its place stands between the count and the end of the loop
//!   ([`place`]). A method call through auto-ref on the receiver or a prefix of it is not read
//!   (a stated limit of root decision R1; see [`place`]).
//! - **Outer loops (D1)** are in [`outer`] and [`select`]: every other loop around a counted drain,
//!   inside its region, must be finite ([`outer`]) and must select the queue, a different one on
//!   each pass, and keep it in place for the pass ([`select`], #1418 D1, R3). The loops of a
//!   per-queue form are judged by [`outer`] alone: each pop there is bounded by its own queue's
//!   counter, however many passes run. **Attributes and items (D2, D9)** are in [`count`], [`outer`], [`per_queue`]
//!   and [`items`]: a count binding under a `cfg`-family attribute, and an item of any kind
//!   (`const`, `static`, `struct`, `enum`, `union`, `fn`, `type`, `trait`, `mod`, or a `use`) that
//!   declares the name of any binding a rule read to bound the pop, anywhere in that binding's
//!   scope, are refused. The reads are the count, its alias source, a slice parameter, the root of
//!   an outer loop's collection (`controls` of `&controls`, `controls.iter_mut()`, `cfg` of
//!   `&cfg.controls`) and the roots `<Q>` and `<K>` of a per-queue form. **Constructors (D3)** are
//!   in [`items`]: `from_fn`, `repeat_with` and `successors` are refused anywhere in a region,
//!   except the token of `core::array::from_fn` and `std::array::from_fn`. A pop in the closure of
//!   either is refused like a pop in any closure (B2a-D1).
//! - **Macros and attributes that can declare an item (D10).** An expansion is invisible to the
//!   item scan, and an item declared in a block shadows a binding of an enclosing block, or a
//!   function's parameter, for the whole block it lands in. So the drain is refused when it holds
//!   a macro invocation in statement or item position (`name!(..);`, `name![..];`, `name! {..}`)
//!   or an item with an attribute other than `cfg`, a lint level or `doc` (a derive and an
//!   attribute macro emit items beside their item, and `cfg_attr` can expand to a derive) anywhere
//!   in the scope of a binding the rules read (B2a-D9 amended, root Amendment 4). The scope of a
//!   `let` (the count, its alias source, or a collection root) is the whole block that holds the
//!   `let`, from its `{`, at any depth: an item before the `let`, after the loop or outside the
//!   markers shadows it as well (`struct controls; let controls = controls;` binds nothing). The
//!   scope of a parameter (a slice parameter, a collection root, a queue root) is the whole `fn`
//!   body, and the scope of a pattern (the `for` that binds a collection root) is the node that
//!   binds it. A `debug_assert!` is exempt while it is trusted ([`trusted`]); a macro in expression
//!   position cannot declare an item and is not read. `debug_assert_eq!` and `debug_assert_ne!` are
//!   not exempt: an accepted over-refusal (widening the exemption needs a root ruling).
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
//! - **Function values (D3, root Amendment 4).** A path expression whose last segment is `try_pop`
//!   is allowed only as the callee of a call (`Consumer::try_pop(control)`). Any other use
//!   (`let pop = Consumer::try_pop;`, an argument, a field, the operand of `.map(..)`) is refused:
//!   the value can be called from a place the walk does not see, and `.map` can be a user trait
//!   method that loops or an iterator adapter that pops once per item. A count is
//!   `<recv>.available_at_entry()[.min(<cap>)]` and a pop is `<recv>.try_pop()`: the
//!   `<recv>.map_or(0, <Type>::available_at_entry)` count form is withdrawn for the same reason.
//! - **Trusted names (D2, D3).** The rule trusts `try_pop`, `available_at_entry` and
//!   `debug_assert!` to mean what they mean in the queue and in the standard library. [`trusted`]
//!   reads every `.rs` file under the roots and refuses a written definition or rename of the two
//!   queue names (an import by a `::` path, such as `pub use crate::loops::try_pop;`, is not
//!   refused) outside the one `fn` of each in `crates/engine/src/realtime/spsc.rs`, a
//!   `debug_assert` that withdraws the trust, `macro_use` (except `#[macro_use] mod`),
//!   `no_implicit_prelude`, `feature(..)`, a `${..}` metavariable expression and any `proc_macro`
//!   identifier.
//!
//! A pop outside every region is no longer a limit (#1443 D1 to D3): every non-test pop under the
//! scan roots must stand in a marked region, where this rule judges it, or in the own body of a
//! function on the control-side allowlist of [`crate::Policy`], with exactly its site count, and
//! every `try_pop` token is classified. [`crate::unmarked`] holds the test-code definition (a file
//! under `<package>/tests/`, a `#[test]` or false-`cfg` node, a file that only test code declares;
//! `#[cfg(any(test, feature = "test-support"))]` is not test code) and the allowlist.
//!
//! A call from a marked loop or closure to a function that pops is the call rule's
//! ([`crate::calls`], #1444): it refuses the call, a path value that names such a function, and an
//! iterator implementation that pops.
//!
//! What stays outside the rule (D4): recursion; a loop in a caller, outside the region (a loop
//! that encloses the region inside the same function is refused above, not a limit); a helper two
//! calls deep; a function value that enters a function as a parameter, a field or a return value;
//! an implicit call other than iteration (`Drop`, `Deref`, an operator trait); and a macro,
//! defined elsewhere, that expands to a loop.
//!
//! The limits of the tool's threat model (#1438 Hazards): code outside the workspace source, which
//! the tool cannot read, type-level behaviour, because the rule reads syntax and not types, and
//! names or paths that a macro expansion builds from parts. The tool judges the workspace's own
//! written source, token by token and inside macro token trees too, so:
//!
//! - **An iterator that never ends, borrowed from a field or a local (type-level).** `&mut rounds`
//!   and `rounds.iter_mut()` are finite outer loops by their form (D1). Since #1418 such a loop
//!   must also select the queue, so `let mut rounds = 0_u32..; for _ in &mut rounds { <counted
//!   drain> }` is refused (`_` selects nothing); a loop whose element is the queue still takes the
//!   collection at its word (D3). For the same reason a `let` of a unit
//!   struct's name (`let controls = controls;` with `struct controls;` at module level) binds
//!   nothing, and its value has that item's type: limit 2, as for a `let` of any type.
//! - **A registry crate (code outside the workspace).** One that exports a macro named
//!   `debug_assert`, a proc macro, or a trait or type with a looping method, is outside the scan
//!   of [`trusted`]. No crate of the workspace does this.
//! - **Auto-deref and impl resolution (type-level).** Which `Deref` target or which impl a method
//!   call resolves to is not read.
//! - **Macro metaprogramming (owner decision of 2026-10-11, limit 4).** An identifier, item name,
//!   path, attribute or file path that a macro expansion builds from parts (a `macro_rules!` or
//!   `macro` fragment or `tt` substituted into an item, attribute, `use` or call position, an
//!   attribute or derive macro) is not modelled. `mkp!(fn x.try_pop())` that expands to a looping
//!   `try_pop` passes. Every direct written form is refused; the written gates `${`, `feature(` and
//!   `proc_macro*` stay refused; the runtime audits (0 allocations, 0 syscalls) are the backstop.
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
mod place;
mod scope;
mod select;
mod shapes;
mod tree;
mod trusted;

use std::cell::RefCell;
use std::collections::BTreeSet;

use crate::boundary::{Boundaries, RegionNodes};
use crate::markers::Region;
use crate::report::{Class, Finding};
use crate::source::Source;
use facts::{Facts, Use};
pub(crate) use facts::{PopKind, PopSite, macro_pops};
pub(crate) use names::bare;
use scope::{Binding, BindingKind, nearest};
use tree::{Node, Step, Tree, Via};
pub(crate) use tree::{Pos, pos};
pub(crate) use trusted::NameScan;

/// What a judgment reads: the file's tree, its facts, the pop's region, and every pop of the file.
struct Ctx<'f> {
    boundaries: &'f Boundaries,
    tree: &'f Tree<'f>,
    facts: &'f Facts,
    region: &'f RegionNodes,
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
            let ctx = Ctx {
                boundaries,
                tree: &tree,
                facts: &facts,
                region: &nodes[index],
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
    if let Some(counted) = count::bounds(ctx, &steps, at) {
        // #1418 D1: and every one of them drains a different queue on each pass.
        return select::selects(ctx, &steps, top, at, &counted)
            && pops_the_counted_queue(ctx, &steps, at, &counted)
            && !shadowed_by_an_item(ctx, &steps);
    }
    per_queue::bounds(ctx, &steps, at, site) && !shadowed_by_an_item(ctx, &steps)
}

/// #1426 D1, D2: whether the pop at the end of `steps`, in the body of the counted loop at
/// `steps[at]`, pops the receiver that the loop counts, and nothing replaces that receiver's place
/// between the count and the end of the loop.
///
/// - **D2.** The pop is `<recv>.try_pop()`. A callee (`Consumer::try_pop(control)`) or a receiver
///   outside the count's grammar (`(control).try_pop()`) cannot be read, and is refused.
/// - **D1.1.** `<recv>` is the counted receiver, token for token.
/// - **D1.2.** Each root of the two (the receiver's own root, and the root of each index place)
///   resolves to the same binding through the one resolver, so a `let` or a pattern that rebinds a
///   root between the count and the pop is refused.
/// - **D1.3.** No use of a root between the count and the end of the loop replaces its place
///   ([`place`]).
fn pops_the_counted_queue<'f>(
    ctx: &Ctx<'f>,
    steps: &[Step<'f>],
    at: usize,
    counted: &count::Counted<'f>,
) -> bool {
    let Some(Step {
        node: Node::Expr(syn::Expr::MethodCall(pop)),
        ..
    }) = steps.last()
    else {
        return false;
    };
    if bare(&pop.method) != facts::POP || pop.turbofish.is_some() || !pop.args.is_empty() {
        return false;
    }
    if !shapes::same_receiver(counted.receiver, &pop.receiver) {
        return false;
    }
    let (Some(counted_roots), Some(popped_roots)) = (
        shapes::receiver_roots(counted.receiver),
        shapes::receiver_roots(&pop.receiver),
    ) else {
        return false;
    };
    let outermost = (0..=at)
        .find(|&i| {
            steps[i].node.is_loop()
                && ctx
                    .tree
                    .extent(steps[i].node)
                    .is_some_and(|extent| extent.core >= counted.start)
        })
        .unwrap_or(at);
    let Some(end) = ctx
        .tree
        .extent(steps[outermost].node)
        .map(|extent| extent.end)
    else {
        return false;
    };
    counted_roots.len() == popped_roots.len()
        && counted_roots
            .iter()
            .zip(&popped_roots)
            .all(|(first, second)| {
                let (Some(counted_binding), Some(popped_binding)) = (
                    ctx.binding_of_path(&first.name, first.at),
                    ctx.binding_of_path(&second.name, second.at),
                ) else {
                    return false;
                };
                counted_binding.same(&popped_binding)
                    && place::untouched(ctx, first, &counted_binding, counted.start, end)
            })
}

/// D9, D10: whether an item, a macro or an attribute can rebind a name that the rules read.
///
/// - **D9.** An item of any kind (`const`, `static`, `struct`, `enum`, `union`, `fn`, `type`,
///   `trait`, `mod`) that declares the name of a binding the rules read, or a `use` that can name
///   it, shadows that binding for the whole block it lands in, wherever it stands. The reads are
///   the count, its alias source, a slice parameter, the root of an outer loop's collection and
///   the roots of a per-queue form.
/// - **D10.** A macro in statement or item position, or an item with an attribute that can emit
///   items, can declare such an item.
///
/// Both are refused anywhere in the scope of the read binding (root Amendment 4, R3): see
/// [`read_scope`]. The scope is the binding's own, not the region's: an item outside the markers
/// but in the same block shadows the count, the parameter or the root just the same.
fn shadowed_by_an_item<'f>(ctx: &Ctx<'f>, steps: &[Step<'f>]) -> bool {
    let reads = ctx.reads.borrow();
    let mut span: Option<(Pos, Pos)> = None;
    for (name, binding) in reads.iter() {
        let Some((from, to)) = read_scope(ctx, steps, binding) else {
            // A binding that cannot be placed is refused: only a fail-closed answer is safe.
            return true;
        };
        if items::names_item(ctx.boundaries.leaves_between(from, to), name) {
            return true;
        }
        span = Some(span.map_or((from, to), |(first, last)| (first.min(from), last.max(to))));
    }
    span.is_some_and(|(from, to)| ctx.facts.declares_items_in(from, to))
}

/// The scope of one read binding (D9, D10; root Amendment 4, R3).
///
/// - A `let`: the whole block that holds the `let`, from its `{`, at any depth, the statements
///   before the `let` included. An item of that block shadows the `let`'s name wherever it stands:
///   `struct controls; let controls = controls;` binds nothing (the `let` is a unit-struct
///   pattern), and `let controls = ..` is the unit struct's value. A `let` whose identifier
///   pattern names a unit struct or unit variant that is in scope binds nothing, and its value has
///   that item's type: that is the type-level limit of the threat model, as for a `let` of any
///   type.
/// - A parameter: the whole `fn` body. A parameter's block is the body, and an item anywhere in
///   the body shadows the parameter: a `const` or a macro that declares one, after the loop, at the
///   body's level included.
/// - A pattern (a `for`, `if let`, `while let`, `match` arm or closure): the whole node that binds
///   it.
fn read_scope<'f>(ctx: &Ctx<'f>, steps: &[Step<'f>], binding: &Binding<'f>) -> Option<(Pos, Pos)> {
    match binding.kind {
        BindingKind::Let { stmt, .. } => {
            let at = steps.iter().position(|step| {
                matches!((step.node, step.via), (Node::Block(block), Some(Via::Stmt(index)))
                    if block.stmts[..index].iter().any(|before| std::ptr::eq(before, stmt)))
            })?;
            let block = ctx.tree.extent(steps[at].node)?;
            Some((block.full, block.end))
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

/// Whether the path expression that ends `steps` stands where it is called at once (B2a-D3, root
/// Amendment 4): as the callee of a call. A function value anywhere else (an argument, a `.map(..)`
/// operand, a `let`, a field) can be called from a place the walk does not see.
fn called_in_place(steps: &[Step<'_>]) -> bool {
    let Some(parent) = steps.len().checked_sub(2).map(|at| steps[at]) else {
        return false;
    };
    matches!(
        (parent.node, parent.via),
        (Node::Expr(syn::Expr::Call(_)), Some(Via::Operand(0)))
    )
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
