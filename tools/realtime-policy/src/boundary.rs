//! Where a region marker may sit: the syntax rule for region boundaries.
//!
//! A marker line must sit at a child boundary of its innermost container: before the first child,
//! between two children, or after the last. A container is the file, the braces of an inline
//! `mod`, an `impl`, a `trait` or an `extern` block, a block, or a group of a macro's token tree.
//! A container's children are its items or statements, or, in a macro's token tree, the token
//! trees at one nesting level of the group.
//!
//! An item or statement starts at its first token after its outer attributes, so a marker between
//! a doc comment and its item is at a child boundary. An attribute is itself whole: a marker
//! inside one splits it. A region whose two markers sit in different containers, one of them a
//! macro token tree, is refused: it would hold some of the macro's tokens and some syntax.

use proc_macro2::LineColumn;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};

use crate::leaf::{self, Kind, Leaf};
use crate::markers::Region;
use crate::report::{Class, Finding};
use crate::source::Source;

/// A file's leaves and container layout, and the innermost container of each of its markers.
pub(crate) struct Boundaries {
    leaves: Leaves,
    layout: Layout,
    /// The innermost container of each marker in source order: each region's begin, then its end.
    innermost: Vec<usize>,
}

impl Boundaries {
    /// Lay out `file`, which parsed from `source`, against the markers of `regions`.
    pub(crate) fn new(source: &Source, file: &syn::File, regions: &[Region]) -> Self {
        let leaves = Leaves::new(&source.tokens);
        let layout = Layout::of(file, &leaves);
        let lines: Vec<usize> = regions
            .iter()
            .flat_map(|region| [region.begin, region.end])
            .collect();
        let innermost = layout.innermost_containers(&lines);
        Self {
            leaves,
            layout,
            innermost,
        }
    }

    /// The first token position at or after `position` once the outer attributes that start there
    /// are skipped; `position` itself when there is no token.
    pub(crate) fn after_attributes(&self, position: LineColumn) -> LineColumn {
        self.leaves
            .after_attributes(position)
            .map_or(position, |leaf| leaf.start)
    }

    /// How many times the identifier `name` (a raw identifier is its bare name) is a token that
    /// starts in `from..to`, in whatever role: a use, a binding, a field, a method.
    pub(crate) fn words_in(&self, name: &str, from: (usize, usize), to: (usize, usize)) -> usize {
        let first = self
            .leaves
            .all
            .partition_point(|leaf| key(leaf.start) < from);
        self.leaves.all[first..]
            .iter()
            .take_while(|leaf| key(leaf.start) < to)
            .filter(|leaf| matches!(&leaf.kind, Kind::Word(word) if word.strip_prefix("r#").unwrap_or(word) == name))
            .count()
    }

    /// The nodes of region number `index` (A2-D1): the maximal children of the file's containers
    /// whose extent lies wholly between its markers. A region with a marker in a macro token tree
    /// is tokens, not syntax: it has no nodes.
    pub(crate) fn region_nodes(&self, index: usize, region: &Region) -> RegionNodes {
        let (begin, end) = (self.innermost[2 * index], self.innermost[2 * index + 1]);
        let tokens_only =
            self.layout.containers[begin].macro_tree || self.layout.containers[end].macro_tree;
        let (begin_at, end_at) = (
            key(marker_position(region.begin)),
            key(marker_position(region.end)),
        );
        let mut inside: Vec<((usize, usize), (usize, usize))> = Vec::new();
        if !tokens_only {
            for container in self.layout.containers.iter().filter(|c| !c.macro_tree) {
                let first = container
                    .children
                    .partition_point(|child| key(child.start) <= begin_at);
                inside.extend(
                    container.children[first..]
                        .iter()
                        .take_while(|child| key(child.start) < end_at)
                        .filter(|child| key(child.end) < end_at)
                        .map(|child| (key(child.start), key(child.end))),
                );
            }
        }
        // The widest first at each start, so a node inside an earlier one is skipped.
        inside.sort_by(|left, right| left.0.cmp(&right.0).then(right.1.cmp(&left.1)));
        let mut nodes: Vec<((usize, usize), (usize, usize))> = Vec::new();
        for node in inside {
            if nodes.last().is_none_or(|last| last.1 <= node.0) {
                nodes.push(node);
            }
        }
        RegionNodes {
            begin: begin_at,
            end: end_at,
            nodes,
        }
    }
}

/// The nodes of one region (A2-D1): its maximal syntax nodes, or nothing when the region is
/// tokens of a macro's token tree.
pub(crate) struct RegionNodes {
    begin: (usize, usize),
    end: (usize, usize),
    /// The maximal nodes' extents, sorted, from the first token after attributes to the end.
    nodes: Vec<((usize, usize), (usize, usize))>,
}

impl RegionNodes {
    /// Whether `start..end` is one of the region's maximal nodes.
    pub(crate) fn is_node(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        self.nodes
            .binary_search_by(|node| node.0.cmp(&start))
            .is_ok_and(|index| self.nodes[index].1 == end)
    }

    /// Whether `start..end` lies wholly between the markers.
    pub(crate) fn holds(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        self.begin < start && end < self.end
    }
}

/// Check the boundaries of `regions`, the regions of `source`.
pub(crate) fn check(
    source: &Source,
    boundaries: &Boundaries,
    regions: &[Region],
    findings: &mut Vec<Finding>,
) {
    let layout = &boundaries.layout;
    let innermost = &boundaries.innermost;
    for (index, region) in regions.iter().enumerate() {
        let (begin, end) = (innermost[2 * index], innermost[2 * index + 1]);
        for (line, container) in [(region.begin, begin), (region.end, end)] {
            if let Some(split) = layout.split_by(line, container) {
                findings.push(Finding::at_line(
                    Class::RegionSplits(split),
                    &source.path,
                    line,
                    source.line_text(line),
                ));
            }
        }
        if begin != end
            && (layout.containers[begin].macro_tree || layout.containers[end].macro_tree)
        {
            findings.push(Finding::at_line(
                Class::RegionSplitsMacroTree,
                &source.path,
                region.begin,
                source.line_text(region.begin),
            ));
        }
    }
}

/// The extent of a child: from its first token after its outer attributes to its end. `None`
/// for a node with no tokens.
fn extent(leaves: &Leaves, node: &impl Spanned) -> Option<(LineColumn, LineColumn)> {
    let span = node.span();
    if span.start().line == 0 {
        return None;
    }
    let first = leaves.after_attributes(span.start())?;
    (key(first.start) < key(span.end())).then_some((first.start, span.end()))
}

/// A position as a sortable pair.
fn key(position: LineColumn) -> (usize, usize) {
    (position.line, position.column)
}

/// The position of the start of a marker line. No token sits on a marker line, so a node holds
/// the marker exactly when it starts before this position and ends after it.
fn marker_position(line: usize) -> LineColumn {
    LineColumn { line, column: 0 }
}

/// A file's leaf tokens in source order, with the closing delimiter of every opening one.
pub(crate) struct Leaves {
    all: Vec<Leaf>,
    /// `close[i]` is the index of the closing delimiter of the opening delimiter `all[i]`.
    close: Vec<usize>,
}

impl Leaves {
    fn new(tokens: &proc_macro2::TokenStream) -> Self {
        let all = leaf::flatten(tokens);
        let mut close = vec![0; all.len()];
        let mut open = Vec::new();
        for (index, leaf) in all.iter().enumerate() {
            match leaf.kind {
                Kind::Open(_) => open.push(index),
                Kind::Close(_) => {
                    if let Some(opening) = open.pop() {
                        close[opening] = index;
                    }
                }
                _ => {}
            }
        }
        Self { all, close }
    }

    /// The leaf at `position`, then past every outer attribute that starts there.
    fn after_attributes(&self, position: LineColumn) -> Option<&Leaf> {
        let mut index = self
            .all
            .partition_point(|leaf| key(leaf.start) < key(position));
        while let (Some(hash), Some(bracket)) = (self.all.get(index), self.all.get(index + 1)) {
            if matches!(hash.kind, Kind::Punct('#')) && matches!(bracket.kind, Kind::Open('[')) {
                index = self.close[index + 1] + 1;
            } else {
                break;
            }
        }
        self.all.get(index)
    }
}

/// A node that a container holds directly.
struct Child {
    start: LineColumn,
    end: LineColumn,
    /// What it is, with its article, for the finding.
    name: &'static str,
}

/// A place where markers may sit between children.
struct Container {
    open: LineColumn,
    close: LineColumn,
    /// A group of a macro's token tree. Its children are tokens, which no marker can split.
    macro_tree: bool,
    /// The container's children in source order. Empty for a macro tree.
    children: Vec<Child>,
}

impl Container {
    fn holds(&self, position: LineColumn) -> bool {
        key(self.open) < key(position) && key(position) < key(self.close)
    }
}

/// The containers and attributes of one file, in source order.
struct Layout {
    containers: Vec<Container>,
    /// The extent of every attribute outside a macro's token tree.
    attributes: Vec<(LineColumn, LineColumn)>,
}

impl Layout {
    fn of(file: &syn::File, leaves: &Leaves) -> Self {
        let mut builder = Builder {
            leaves,
            layout: Self {
                containers: Vec::new(),
                attributes: Vec::new(),
            },
        };
        builder.visit_file(file);
        let mut layout = builder.layout;
        // The visit follows the tree's field order, which is not always source order (an inner
        // attribute comes before a parameter's), so the lists are put in source order here.
        layout
            .containers
            .sort_by_key(|container| key(container.open));
        layout.attributes.sort_by_key(|(start, _)| key(*start));
        layout
    }

    /// The innermost container of each marker line in `lines`, which are in source order.
    fn innermost_containers(&self, lines: &[usize]) -> Vec<usize> {
        // The containers are nested in source order, so a stack of the open ones answers each
        // marker after the one before it.
        let mut stack: Vec<usize> = Vec::new();
        let mut next = 0;
        lines
            .iter()
            .map(|&line| {
                let position = marker_position(line);
                while next < self.containers.len()
                    && key(self.containers[next].open) < key(position)
                {
                    let opening = self.containers[next].open;
                    while stack
                        .last()
                        .is_some_and(|&top| key(self.containers[top].close) <= key(opening))
                    {
                        stack.pop();
                    }
                    stack.push(next);
                    next += 1;
                }
                while stack
                    .last()
                    .is_some_and(|&top| !self.containers[top].holds(position))
                {
                    stack.pop();
                }
                *stack
                    .last()
                    .expect("the file container holds every marker line")
            })
            .collect()
    }

    /// What the marker on `line`, whose innermost container is `container`, splits, if anything.
    fn split_by(&self, line: usize, container: usize) -> Option<&'static str> {
        let position = marker_position(line);
        let children = &self.containers[container].children;
        let before = children.partition_point(|child| key(child.start) < key(position));
        if let Some(child) = before.checked_sub(1).map(|index| &children[index])
            && key(position) < key(child.end)
        {
            return Some(child.name);
        }
        let before = self
            .attributes
            .partition_point(|(start, _)| key(*start) < key(position));
        if let Some((_, end)) = before.checked_sub(1).map(|index| &self.attributes[index])
            && key(position) < key(*end)
        {
            return Some("an attribute");
        }
        None
    }
}

/// Walks the tree and records its containers and attributes.
struct Builder<'a> {
    leaves: &'a Leaves,
    layout: Layout,
}

impl Builder<'_> {
    fn child(&self, node: &impl Spanned, name: &'static str) -> Option<Child> {
        extent(self.leaves, node).map(|(start, end)| Child { start, end, name })
    }

    fn push(&mut self, open: LineColumn, close: LineColumn, children: Vec<Child>) {
        self.layout.containers.push(Container {
            open,
            close,
            macro_tree: false,
            children,
        });
    }

    /// Record `group` and every group inside it as macro token trees.
    fn macro_tree(&mut self, tokens: &proc_macro2::TokenStream) {
        for token in tokens.clone() {
            if let proc_macro2::TokenTree::Group(group) = token {
                self.layout.containers.push(Container {
                    open: group.span_open().start(),
                    close: group.span_close().start(),
                    macro_tree: true,
                    children: Vec::new(),
                });
                self.macro_tree(&group.stream());
            }
        }
    }
}

impl<'ast> Visit<'ast> for Builder<'_> {
    fn visit_file(&mut self, node: &'ast syn::File) {
        let children = node
            .items
            .iter()
            .filter_map(|item| self.child(item, item_name(item)))
            .collect();
        self.push(
            LineColumn { line: 0, column: 0 },
            LineColumn {
                line: usize::MAX,
                column: 0,
            },
            children,
        );
        visit::visit_file(self, node);
    }

    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        if let Some((brace, items)) = &node.content {
            let children = items
                .iter()
                .filter_map(|item| self.child(item, item_name(item)))
                .collect();
            self.push(
                brace.span.open().start(),
                brace.span.close().start(),
                children,
            );
        }
        visit::visit_item_mod(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        let children = node
            .items
            .iter()
            .filter_map(|item| self.child(item, impl_item_name(item)))
            .collect();
        self.push(
            node.brace_token.span.open().start(),
            node.brace_token.span.close().start(),
            children,
        );
        visit::visit_item_impl(self, node);
    }

    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        let children = node
            .items
            .iter()
            .filter_map(|item| self.child(item, trait_item_name(item)))
            .collect();
        self.push(
            node.brace_token.span.open().start(),
            node.brace_token.span.close().start(),
            children,
        );
        visit::visit_item_trait(self, node);
    }

    fn visit_item_foreign_mod(&mut self, node: &'ast syn::ItemForeignMod) {
        let children = node
            .items
            .iter()
            .filter_map(|item| self.child(item, foreign_item_name(item)))
            .collect();
        self.push(
            node.brace_token.span.open().start(),
            node.brace_token.span.close().start(),
            children,
        );
        visit::visit_item_foreign_mod(self, node);
    }

    fn visit_block(&mut self, node: &'ast syn::Block) {
        let children = node
            .stmts
            .iter()
            .filter_map(|stmt| self.child(stmt, stmt_name(stmt)))
            .collect();
        self.push(
            node.brace_token.span.open().start(),
            node.brace_token.span.close().start(),
            children,
        );
        visit::visit_block(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        let span = node.delimiter.span();
        self.layout.containers.push(Container {
            open: span.open().start(),
            close: span.close().start(),
            macro_tree: true,
            children: Vec::new(),
        });
        self.macro_tree(&node.tokens);
        visit::visit_macro(self, node);
    }

    fn visit_attribute(&mut self, node: &'ast syn::Attribute) {
        self.layout.attributes.push((
            node.pound_token.span.start(),
            node.bracket_token.span.close().end(),
        ));
        visit::visit_attribute(self, node);
    }
}

fn item_name(item: &syn::Item) -> &'static str {
    match item {
        syn::Item::Const(_) => "a const item",
        syn::Item::Enum(_) => "an enum",
        syn::Item::ExternCrate(_) => "an extern crate item",
        syn::Item::Fn(_) => "a function",
        syn::Item::ForeignMod(_) => "an extern block",
        syn::Item::Impl(_) => "an impl",
        syn::Item::Macro(_) => "a macro item",
        syn::Item::Mod(_) => "a module",
        syn::Item::Static(_) => "a static item",
        syn::Item::Struct(_) => "a struct",
        syn::Item::Trait(_) => "a trait",
        syn::Item::TraitAlias(_) => "a trait alias",
        syn::Item::Type(_) => "a type alias",
        syn::Item::Union(_) => "a union",
        syn::Item::Use(_) => "a use declaration",
        _ => "an item",
    }
}

fn impl_item_name(item: &syn::ImplItem) -> &'static str {
    match item {
        syn::ImplItem::Const(_) => "an associated const",
        syn::ImplItem::Fn(_) => "a method",
        syn::ImplItem::Type(_) => "an associated type",
        syn::ImplItem::Macro(_) => "a macro item",
        _ => "an impl item",
    }
}

fn trait_item_name(item: &syn::TraitItem) -> &'static str {
    match item {
        syn::TraitItem::Const(_) => "an associated const",
        syn::TraitItem::Fn(_) => "a method",
        syn::TraitItem::Type(_) => "an associated type",
        syn::TraitItem::Macro(_) => "a macro item",
        _ => "a trait item",
    }
}

fn foreign_item_name(item: &syn::ForeignItem) -> &'static str {
    match item {
        syn::ForeignItem::Fn(_) => "a foreign function",
        syn::ForeignItem::Static(_) => "a foreign static",
        syn::ForeignItem::Type(_) => "a foreign type",
        syn::ForeignItem::Macro(_) => "a macro item",
        _ => "a foreign item",
    }
}

fn stmt_name(stmt: &syn::Stmt) -> &'static str {
    match stmt {
        syn::Stmt::Local(_) => "a let statement",
        syn::Stmt::Item(item) => item_name(item),
        syn::Stmt::Expr(expr, _) => expr_name(expr),
        syn::Stmt::Macro(_) => "a macro invocation",
    }
}

fn expr_name(expr: &syn::Expr) -> &'static str {
    match expr {
        syn::Expr::Array(_) => "an array expression",
        syn::Expr::Assign(_) => "an assignment",
        syn::Expr::Async(_) => "an async block",
        syn::Expr::Await(_) => "an await expression",
        syn::Expr::Binary(_) => "a binary expression",
        syn::Expr::Block(_) => "a block expression",
        syn::Expr::Break(_) => "a break expression",
        syn::Expr::Call(_) => "a call expression",
        syn::Expr::Cast(_) => "a cast expression",
        syn::Expr::Closure(_) => "a closure",
        syn::Expr::Const(_) => "a const block",
        syn::Expr::Continue(_) => "a continue expression",
        syn::Expr::Field(_) => "a field access",
        syn::Expr::ForLoop(_) => "a for loop",
        syn::Expr::If(_) => "an if expression",
        syn::Expr::Index(_) => "an index expression",
        syn::Expr::Let(_) => "a let condition",
        syn::Expr::Lit(_) => "a literal",
        syn::Expr::Loop(_) => "a loop",
        syn::Expr::Macro(_) => "a macro invocation",
        syn::Expr::Match(_) => "a match expression",
        syn::Expr::MethodCall(_) => "a method call",
        syn::Expr::Paren(_) => "a parenthesized expression",
        syn::Expr::Path(_) => "a path expression",
        syn::Expr::Range(_) => "a range expression",
        syn::Expr::RawAddr(_) => "a raw address expression",
        syn::Expr::Reference(_) => "a reference expression",
        syn::Expr::Repeat(_) => "an array repeat expression",
        syn::Expr::Return(_) => "a return expression",
        syn::Expr::Struct(_) => "a struct expression",
        syn::Expr::Try(_) => "a try expression",
        syn::Expr::TryBlock(_) => "a try block",
        syn::Expr::Tuple(_) => "a tuple expression",
        syn::Expr::Unary(_) => "a unary expression",
        syn::Expr::Unsafe(_) => "an unsafe block",
        syn::Expr::While(_) => "a while loop",
        syn::Expr::Yield(_) => "a yield expression",
        _ => "an expression",
    }
}
