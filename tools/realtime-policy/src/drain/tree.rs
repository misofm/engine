//! The syntax skeleton that the drain rule walks: the nodes of D4's allow-list, found by position.
//!
//! A [`Node`] is a reference into the parsed file (or, for a `debug_assert!`, into its parsed
//! arguments). [`Tree::children`] lists the children of a node that the allow-list admits and
//! the [`Via`] slot each one fills, and [`Tree::descend`] follows them down to a source position.
//! A node outside the list has no entry here, so a pop below it is never reached and is refused.

use std::cell::RefCell;
use std::collections::HashMap;

use proc_macro2::LineColumn;
use syn::spanned::Spanned;

use super::names::bare;
use crate::boundary::Boundaries;

/// A source position as a sortable pair of line and column.
pub(super) type Pos = (usize, usize);

/// A position as a sortable pair.
pub(super) fn pos(position: LineColumn) -> Pos {
    (position.line, position.column)
}

/// The place of a node: `full` is its first token (an outer attribute counts), `core` its first
/// token after its outer attributes, and `end` one past its last character.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Extent {
    pub(super) full: Pos,
    pub(super) core: Pos,
    pub(super) end: Pos,
}

/// A node of the allow-list (D4): the file; the items `fn`, `impl`, `mod` and `trait` with their
/// methods; blocks; statements; the initializer of a `let` with its `else` block; match arms;
/// struct-expression fields; and the expressions that D4 names.
#[derive(Clone, Copy)]
pub(super) enum Node<'f> {
    File(&'f syn::File),
    Item(&'f syn::Item),
    ImplFn(&'f syn::ImplItemFn),
    TraitFn(&'f syn::TraitItemFn),
    Block(&'f syn::Block),
    Stmt(&'f syn::Stmt),
    Init(&'f syn::LocalInit),
    Expr(&'f syn::Expr),
    Arm(&'f syn::Arm),
    Field(&'f syn::FieldValue),
}

impl<'f> Node<'f> {
    /// The node's kind and address: two nodes are the same node when these are equal.
    pub(super) fn id(self) -> (u8, usize) {
        match self {
            Self::File(node) => (0, std::ptr::from_ref(node) as usize),
            Self::Item(node) => (1, std::ptr::from_ref(node) as usize),
            Self::ImplFn(node) => (2, std::ptr::from_ref(node) as usize),
            Self::TraitFn(node) => (3, std::ptr::from_ref(node) as usize),
            Self::Block(node) => (4, std::ptr::from_ref(node) as usize),
            Self::Stmt(node) => (5, std::ptr::from_ref(node) as usize),
            Self::Init(node) => (6, std::ptr::from_ref(node) as usize),
            Self::Expr(node) => (7, std::ptr::from_ref(node) as usize),
            Self::Arm(node) => (8, std::ptr::from_ref(node) as usize),
            Self::Field(node) => (9, std::ptr::from_ref(node) as usize),
        }
    }

    /// Whether the node is a `for`, `while` or `loop` expression.
    pub(super) fn is_loop(self) -> bool {
        matches!(
            self,
            Self::Expr(syn::Expr::ForLoop(_) | syn::Expr::While(_) | syn::Expr::Loop(_))
        )
    }

    /// Whether the node is a function: an item `fn`, a method, or a trait method.
    pub(super) fn is_function(self) -> bool {
        matches!(
            self,
            Self::Item(syn::Item::Fn(_)) | Self::ImplFn(_) | Self::TraitFn(_)
        )
    }

    /// The macro call a node holds, if it is one.
    pub(super) fn macro_call(self) -> Option<&'f syn::Macro> {
        match self {
            Self::Expr(syn::Expr::Macro(call)) => Some(&call.mac),
            Self::Stmt(syn::Stmt::Macro(call)) => Some(&call.mac),
            _ => None,
        }
    }
}

/// Whether the step `at` of a path is a loop of the drain rule: a `for`, `while` or `loop`, or the
/// closure argument of a call whose callee ends in `from_fn`, which runs that closure repeatedly.
pub(super) fn is_loop_at(steps: &[Step<'_>], at: usize) -> bool {
    steps[at].node.is_loop() || from_fn_call(steps, at).is_some()
}

/// The call whose closure argument is the step `at`, when its callee's last segment is `from_fn`.
pub(super) fn from_fn_call<'f>(steps: &[Step<'f>], at: usize) -> Option<&'f syn::ExprCall> {
    let Node::Expr(syn::Expr::Closure(_)) = steps.get(at)?.node else {
        return None;
    };
    let Step {
        node: Node::Expr(syn::Expr::Call(call)),
        via: Some(Via::Operand(slot)),
    } = steps[at.checked_sub(1)?]
    else {
        return None;
    };
    let syn::Expr::Path(callee) = &*call.func else {
        return None;
    };
    (slot >= 1
        && callee
            .path
            .segments
            .last()
            .is_some_and(|last| bare(&last.ident) == "from_fn"))
    .then_some(call)
}

/// The slot of a parent that a child fills.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Via {
    /// The `n`th item of a file, module, `impl` or `trait`.
    Item(usize),
    /// The `n`th statement of a block.
    Stmt(usize),
    /// The body of a function, loop or closure, a match arm's body, or the block of `unsafe`
    /// and of a bare block.
    Body,
    /// The initializer of a `let`.
    Init,
    /// The `else` block of a `let .. else`.
    Diverge,
    /// The header of a loop: a `while` condition or a `for` iterator.
    Header,
    /// The condition of an `if`.
    Cond,
    /// The `then` block of an `if`.
    Then,
    /// The `else` branch of an `if`.
    Else,
    /// The scrutinee of a `match`.
    Scrutinee,
    /// The `n`th arm of a `match`.
    Arm(usize),
    /// An arm's guard.
    Guard,
    /// The `n`th operand of an expression, in source order (a call's callee is 0).
    Operand(usize),
    /// The `n`th parsed argument of a `debug_assert!`.
    MacroArg(usize),
}

/// A node on a path from the file down, and the slot of it that the path continues through.
#[derive(Clone, Copy)]
pub(super) struct Step<'f> {
    pub(super) node: Node<'f>,
    /// `None` on the last step of a path.
    pub(super) via: Option<Via>,
}

/// The file's syntax skeleton with the extents of the nodes it is asked about.
pub(super) struct Tree<'f> {
    file: &'f syn::File,
    boundaries: &'f Boundaries,
    debug_args: &'f HashMap<Pos, Vec<syn::Expr>>,
    extents: RefCell<HashMap<(u8, usize), Option<Extent>>>,
}

impl<'f> Tree<'f> {
    pub(super) fn new(
        file: &'f syn::File,
        boundaries: &'f Boundaries,
        debug_args: &'f HashMap<Pos, Vec<syn::Expr>>,
    ) -> Self {
        Self {
            file,
            boundaries,
            debug_args,
            extents: RefCell::new(HashMap::new()),
        }
    }

    /// The first node of every path.
    pub(super) fn root(&self) -> Node<'f> {
        Node::File(self.file)
    }

    /// The extent of `node`, or `None` for a node with no tokens. The file has none.
    pub(super) fn extent(&self, node: Node<'f>) -> Option<Extent> {
        let id = node.id();
        if let Some(found) = self.extents.borrow().get(&id) {
            return *found;
        }
        let found = match node {
            Node::File(_) => None,
            Node::Item(item) => self.measure(item),
            Node::ImplFn(item) => self.measure(item),
            Node::TraitFn(item) => self.measure(item),
            Node::Block(block) => self.measure(block),
            Node::Stmt(stmt) => self.measure(stmt),
            Node::Init(init) => Some(init_extent(init)),
            Node::Expr(expr) => self.measure(expr),
            Node::Arm(arm) => self.measure(arm),
            Node::Field(field) => self.measure(field),
        };
        self.extents.borrow_mut().insert(id, found);
        found
    }

    fn measure(&self, node: &impl Spanned) -> Option<Extent> {
        let span = node.span();
        if span.start().line == 0 {
            return None;
        }
        let full = pos(span.start());
        let core = pos(self.boundaries.after_attributes(span.start()));
        Some(Extent {
            full,
            core,
            end: pos(span.end()),
        })
    }

    /// The children of `node` that D4 admits, in source order, with the slot each one fills.
    pub(super) fn children(&self, node: Node<'f>) -> Vec<(Via, Node<'f>)> {
        let mut out = Vec::new();
        match node {
            Node::File(file) => items(&file.items, &mut out),
            Node::Item(item) => match item {
                syn::Item::Fn(function) => out.push((Via::Body, Node::Block(&function.block))),
                syn::Item::Impl(imp) => {
                    for (index, member) in imp.items.iter().enumerate() {
                        if let syn::ImplItem::Fn(function) = member {
                            out.push((Via::Item(index), Node::ImplFn(function)));
                        }
                    }
                }
                syn::Item::Mod(module) => {
                    if let Some((_, content)) = &module.content {
                        items(content, &mut out);
                    }
                }
                syn::Item::Trait(item_trait) => {
                    for (index, member) in item_trait.items.iter().enumerate() {
                        if let syn::TraitItem::Fn(function) = member {
                            out.push((Via::Item(index), Node::TraitFn(function)));
                        }
                    }
                }
                _ => {}
            },
            Node::ImplFn(function) => out.push((Via::Body, Node::Block(&function.block))),
            Node::TraitFn(function) => {
                if let Some(block) = &function.default {
                    out.push((Via::Body, Node::Block(block)));
                }
            }
            Node::Block(block) => {
                for (index, stmt) in block.stmts.iter().enumerate() {
                    out.push((Via::Stmt(index), Node::Stmt(stmt)));
                }
            }
            Node::Stmt(stmt) => match stmt {
                syn::Stmt::Local(local) => {
                    if let Some(init) = &local.init {
                        out.push((Via::Init, Node::Init(init)));
                    }
                }
                syn::Stmt::Expr(expr, _) => out.push((Via::Operand(0), Node::Expr(expr))),
                syn::Stmt::Item(item) => {
                    if matches!(
                        item,
                        syn::Item::Fn(_)
                            | syn::Item::Impl(_)
                            | syn::Item::Mod(_)
                            | syn::Item::Trait(_)
                    ) {
                        out.push((Via::Operand(0), Node::Item(item)));
                    }
                }
                syn::Stmt::Macro(call) => self.macro_children(&call.mac, &mut out),
            },
            Node::Init(init) => {
                out.push((Via::Operand(0), Node::Expr(&init.expr)));
                if let Some((_, diverge)) = &init.diverge {
                    out.push((Via::Diverge, Node::Expr(diverge)));
                }
            }
            Node::Expr(expr) => self.expr_children(expr, &mut out),
            Node::Arm(arm) => {
                if let Some((_, guard)) = &arm.guard {
                    out.push((Via::Guard, Node::Expr(guard)));
                }
                out.push((Via::Body, Node::Expr(&arm.body)));
            }
            Node::Field(field) => out.push((Via::Operand(0), Node::Expr(&field.expr))),
        }
        out
    }

    /// A `debug_assert!`'s arguments, parsed as expressions, are its children.
    fn macro_children(&self, call: &'f syn::Macro, out: &mut Vec<(Via, Node<'f>)>) {
        if let Some(args) = self.debug_args.get(&macro_key(call)) {
            out.extend(
                args.iter()
                    .enumerate()
                    .map(|(index, arg)| (Via::MacroArg(index), Node::Expr(arg))),
            );
        }
    }

    fn expr_children(&self, expr: &'f syn::Expr, out: &mut Vec<(Via, Node<'f>)>) {
        use syn::Expr as E;
        let operands = |out: &mut Vec<(Via, Node<'f>)>, list: &[&'f syn::Expr]| {
            out.extend(
                list.iter()
                    .enumerate()
                    .map(|(index, operand)| (Via::Operand(index), Node::Expr(operand))),
            );
        };
        match expr {
            E::Array(node) => operands(out, &node.elems.iter().collect::<Vec<_>>()),
            E::Tuple(node) => operands(out, &node.elems.iter().collect::<Vec<_>>()),
            E::Assign(node) => operands(out, &[&node.left, &node.right]),
            E::Binary(node) => operands(out, &[&node.left, &node.right]),
            E::Block(node) => out.push((Via::Body, Node::Block(&node.block))),
            E::Unsafe(node) => out.push((Via::Body, Node::Block(&node.block))),
            E::Break(node) => {
                if let Some(value) = &node.expr {
                    operands(out, &[value]);
                }
            }
            E::Return(node) => {
                if let Some(value) = &node.expr {
                    operands(out, &[value]);
                }
            }
            E::Call(node) => {
                let mut all: Vec<&syn::Expr> = vec![&node.func];
                all.extend(node.args.iter());
                operands(out, &all);
            }
            E::MethodCall(node) => {
                let mut all: Vec<&syn::Expr> = vec![&node.receiver];
                all.extend(node.args.iter());
                operands(out, &all);
            }
            E::Cast(node) => operands(out, &[&node.expr]),
            E::Closure(node) => out.push((Via::Body, Node::Expr(&node.body))),
            E::Field(node) => operands(out, &[&node.base]),
            E::Group(node) => operands(out, &[&node.expr]),
            E::Paren(node) => operands(out, &[&node.expr]),
            E::Index(node) => operands(out, &[&node.expr, &node.index]),
            E::Let(node) => operands(out, &[&node.expr]),
            E::Range(node) => {
                if let Some(start) = &node.start {
                    out.push((Via::Operand(0), Node::Expr(start)));
                }
                if let Some(end) = &node.end {
                    out.push((Via::Operand(1), Node::Expr(end)));
                }
            }
            E::RawAddr(node) => operands(out, &[&node.expr]),
            E::Reference(node) => operands(out, &[&node.expr]),
            E::Repeat(node) => operands(out, &[&node.expr, &node.len]),
            E::Try(node) => operands(out, &[&node.expr]),
            E::Unary(node) => operands(out, &[&node.expr]),
            E::Struct(node) => {
                for (index, field) in node.fields.iter().enumerate() {
                    out.push((Via::Operand(index), Node::Field(field)));
                }
                if let Some(rest) = &node.rest {
                    out.push((Via::Operand(node.fields.len()), Node::Expr(rest)));
                }
            }
            E::ForLoop(node) => {
                out.push((Via::Header, Node::Expr(&node.expr)));
                out.push((Via::Body, Node::Block(&node.body)));
            }
            E::While(node) => {
                out.push((Via::Header, Node::Expr(&node.cond)));
                out.push((Via::Body, Node::Block(&node.body)));
            }
            E::Loop(node) => out.push((Via::Body, Node::Block(&node.body))),
            E::If(node) => {
                out.push((Via::Cond, Node::Expr(&node.cond)));
                out.push((Via::Then, Node::Block(&node.then_branch)));
                if let Some((_, otherwise)) = &node.else_branch {
                    out.push((Via::Else, Node::Expr(otherwise)));
                }
            }
            E::Match(node) => {
                out.push((Via::Scrutinee, Node::Expr(&node.expr)));
                for (index, arm) in node.arms.iter().enumerate() {
                    out.push((Via::Arm(index), Node::Arm(arm)));
                }
            }
            E::Macro(node) => self.macro_children(&node.mac, out),
            // A leaf: no pop can sit below it.
            E::Lit(_) | E::Path(_) | E::Continue(_) => {}
            // Not on D4's list: `Async`, `Await`, `Const`, `Infer`, `TryBlock`, `Yield`,
            // `Verbatim` and any variant a later `syn` adds have no children here.
            _ => {}
        }
    }

    /// The path from the file down to the deepest admitted node that contains `at`.
    pub(super) fn descend(&self, at: Pos) -> Vec<Step<'f>> {
        let mut steps = Vec::new();
        let mut node = self.root();
        loop {
            let next = self.children(node).into_iter().find(|(_, child)| {
                self.extent(*child)
                    .is_some_and(|extent| extent.full <= at && at < extent.end)
            });
            match next {
                Some((via, child)) => {
                    steps.push(Step {
                        node,
                        via: Some(via),
                    });
                    node = child;
                }
                None => {
                    steps.push(Step { node, via: None });
                    return steps;
                }
            }
        }
    }
}

fn items<'f>(list: &'f [syn::Item], out: &mut Vec<(Via, Node<'f>)>) {
    for (index, item) in list.iter().enumerate() {
        if matches!(
            item,
            syn::Item::Fn(_) | syn::Item::Impl(_) | syn::Item::Mod(_) | syn::Item::Trait(_)
        ) {
            out.push((Via::Item(index), Node::Item(item)));
        }
    }
}

/// The key of a macro call in the table of parsed `debug_assert!` arguments: where its delimiter
/// opens.
pub(super) fn macro_key(call: &syn::Macro) -> Pos {
    pos(call.delimiter.span().open().start())
}

/// Whether `call` is `debug_assert!` (the last path segment, as the awk reads the word before the
/// `!`).
pub(super) fn is_debug_assert(call: &syn::Macro) -> bool {
    call.path
        .segments
        .last()
        .is_some_and(|segment| bare(&segment.ident) == "debug_assert")
}

/// The extent of a `let` initializer: from its `=` to the end of its expression or `else` block.
fn init_extent(init: &syn::LocalInit) -> Extent {
    let start = pos(init.eq_token.span.start());
    let last = init
        .diverge
        .as_ref()
        .map_or_else(|| init.expr.span(), |(_, diverge)| diverge.span());
    Extent {
        full: start,
        core: start,
        end: pos(last.end()),
    }
}
