//! One walk over a non-test file: its functions, its non-test pops with the function around each,
//! and the `try_pop` tokens that test code and `fn try_pop` definitions account for.

use std::collections::BTreeSet;

use syn::spanned::Spanned;
use syn::visit::{self, Visit};

use super::cfg::is_test;
use crate::drain::{PopKind, PopSite, Pos, bare, macro_pops, pos};

/// A function of the file that is not test code.
pub(super) struct Function {
    /// The canonical path (#1443 D3).
    pub(super) path: String,
    /// The line of its name.
    pub(super) line: usize,
}

/// A non-test pop and where it stands.
pub(super) struct Pop {
    pub(super) site: PopSite,
    /// The innermost function around it, as an index into [`Walk::functions`].
    pub(super) function: Option<usize>,
    /// Whether a closure or an `async` block lies between the pop and that function.
    pub(super) deferred: bool,
}

/// What one walk found.
#[derive(Default)]
pub(super) struct Walk {
    pub(super) functions: Vec<Function>,
    pub(super) pops: Vec<Pop>,
    /// The extents (start, one past the end) of the nodes that are test code.
    pub(super) test_ranges: Vec<(Pos, Pos)>,
    /// The names of `fn try_pop` definitions that are not test code.
    pub(super) definitions: BTreeSet<Pos>,
}

/// Walk `file`.
pub(super) fn walk(file: &syn::File) -> Walk {
    let mut walker = Walker {
        found: Walk::default(),
        modules: Vec::new(),
        owner: None,
        frames: Vec::new(),
    };
    walker.visit_file(file);
    walker.found
}

/// A function being walked.
struct Frame {
    index: usize,
    /// How many closures and `async` blocks the walk is inside, within this function.
    deferred: usize,
}

struct Walker {
    found: Walk,
    /// The inline modules around the node, outermost first.
    modules: Vec<String>,
    /// `Type` or `Trait for Type` of the `impl` (or `Trait` of the `trait`) around the node.
    owner: Option<String>,
    /// The functions around the node, outermost first.
    frames: Vec<Frame>,
}

impl Walker {
    /// Account for a node that is test code: the `try_pop` tokens inside its extent are test
    /// code. A node with no place cannot be measured and so accounts for nothing.
    fn test_node(&mut self, node: &impl Spanned) {
        let span = node.span();
        if span.start().line != 0 {
            self.found
                .test_ranges
                .push((pos(span.start()), pos(span.end())));
        }
    }

    /// The canonical path of a function named `name` here (#1443 D3). A function nested in a
    /// function is `<outer>::<name>`.
    fn function_path(&self, name: &str) -> String {
        if self.owner.is_none()
            && let Some(outer) = self.frames.last()
        {
            return format!("{}::{name}", self.found.functions[outer.index].path);
        }
        let mut path = String::new();
        for module in &self.modules {
            path.push_str(module);
            path.push_str("::");
        }
        if let Some(owner) = &self.owner {
            path.push_str(owner);
            path.push_str("::");
        }
        path.push_str(name);
        path
    }

    /// Walk a function: `visit` walks its signature and body inside its frame.
    fn function(&mut self, signature: &syn::Signature, visit_body: impl FnOnce(&mut Self)) {
        let name = bare(&signature.ident);
        if name == "try_pop" {
            self.found
                .definitions
                .insert(pos(signature.ident.span().start()));
        }
        let index = self.found.functions.len();
        let path = self.function_path(&name);
        self.found.functions.push(Function {
            path,
            line: signature.ident.span().start().line,
        });
        self.frames.push(Frame { index, deferred: 0 });
        let owner = self.owner.take();
        visit_body(self);
        self.owner = owner;
        self.frames.pop();
    }

    fn record(&mut self, site: PopSite) {
        let frame = self.frames.last();
        self.found.pops.push(Pop {
            site,
            function: frame.map(|frame| frame.index),
            deferred: frame.is_some_and(|frame| frame.deferred > 0),
        });
    }

    fn deferred(&mut self, enter: bool) {
        if let Some(frame) = self.frames.last_mut() {
            if enter {
                frame.deferred += 1;
            } else {
                frame.deferred -= 1;
            }
        }
    }
}

/// Every `try_pop` identifier of `tokens` at any depth.
pub(super) fn identifiers(tokens: &proc_macro2::TokenStream, found: &mut Vec<Pos>) {
    for token in tokens.clone() {
        match token {
            proc_macro2::TokenTree::Ident(ident) if bare(&ident) == "try_pop" => {
                found.push(pos(ident.span().start()));
            }
            proc_macro2::TokenTree::Group(group) => identifiers(&group.stream(), found),
            _ => {}
        }
    }
}

/// Whether `tokens` hold a `mod name;` item at any depth.
pub(super) fn declares_module(tokens: &proc_macro2::TokenStream) -> bool {
    let list: Vec<proc_macro2::TokenTree> = tokens.clone().into_iter().collect();
    list.windows(3).any(|window| {
        matches!(&window[0], proc_macro2::TokenTree::Ident(word) if word == "mod")
            && matches!(&window[1], proc_macro2::TokenTree::Ident(_))
            && matches!(&window[2], proc_macro2::TokenTree::Punct(semi) if semi.as_char() == ';')
    }) || list.iter().any(|token| match token {
        proc_macro2::TokenTree::Group(group) => declares_module(&group.stream()),
        _ => false,
    })
}

impl<'ast> Visit<'ast> for Walker {
    fn visit_item(&mut self, node: &'ast syn::Item) {
        if is_test(item_attributes(node)) {
            self.test_node(node);
        } else {
            visit::visit_item(self, node);
        }
    }

    fn visit_impl_item(&mut self, node: &'ast syn::ImplItem) {
        let attributes = match node {
            syn::ImplItem::Const(inner) => &inner.attrs,
            syn::ImplItem::Fn(inner) => &inner.attrs,
            syn::ImplItem::Type(inner) => &inner.attrs,
            syn::ImplItem::Macro(inner) => &inner.attrs,
            _ => &[][..],
        };
        if is_test(attributes) {
            self.test_node(node);
        } else {
            visit::visit_impl_item(self, node);
        }
    }

    fn visit_trait_item(&mut self, node: &'ast syn::TraitItem) {
        let attributes = match node {
            syn::TraitItem::Const(inner) => &inner.attrs,
            syn::TraitItem::Fn(inner) => &inner.attrs,
            syn::TraitItem::Type(inner) => &inner.attrs,
            syn::TraitItem::Macro(inner) => &inner.attrs,
            _ => &[][..],
        };
        if is_test(attributes) {
            self.test_node(node);
        } else {
            visit::visit_trait_item(self, node);
        }
    }

    fn visit_stmt(&mut self, node: &'ast syn::Stmt) {
        let attributes = match node {
            syn::Stmt::Local(inner) => &inner.attrs,
            syn::Stmt::Macro(inner) => &inner.attrs,
            _ => &[][..],
        };
        if is_test(attributes) {
            self.test_node(node);
        } else {
            visit::visit_stmt(self, node);
        }
    }

    fn visit_expr(&mut self, node: &'ast syn::Expr) {
        if is_test(expr_attributes(node)) {
            self.test_node(node);
        } else {
            visit::visit_expr(self, node);
        }
    }

    fn visit_arm(&mut self, node: &'ast syn::Arm) {
        if is_test(&node.attrs) {
            self.test_node(node);
        } else {
            visit::visit_arm(self, node);
        }
    }

    fn visit_field_value(&mut self, node: &'ast syn::FieldValue) {
        if is_test(&node.attrs) {
            self.test_node(node);
        } else {
            visit::visit_field_value(self, node);
        }
    }

    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        if node.content.is_some() {
            self.modules.push(bare(&node.ident));
            visit::visit_item_mod(self, node);
            self.modules.pop();
        }
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        let target = type_name(&node.self_ty);
        let owner = match &node.trait_ {
            Some((_, path, _)) => format!("{} for {target}", path_name(path)),
            None => target,
        };
        let outer = self.owner.replace(owner);
        visit::visit_item_impl(self, node);
        self.owner = outer;
    }

    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        let outer = self.owner.replace(bare(&node.ident));
        visit::visit_item_trait(self, node);
        self.owner = outer;
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.function(&node.sig, |this| visit::visit_item_fn(this, node));
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        self.function(&node.sig, |this| visit::visit_impl_item_fn(this, node));
    }

    fn visit_trait_item_fn(&mut self, node: &'ast syn::TraitItemFn) {
        self.function(&node.sig, |this| visit::visit_trait_item_fn(this, node));
    }

    fn visit_expr_closure(&mut self, node: &'ast syn::ExprClosure) {
        self.deferred(true);
        visit::visit_expr_closure(self, node);
        self.deferred(false);
    }

    fn visit_expr_async(&mut self, node: &'ast syn::ExprAsync) {
        self.deferred(true);
        visit::visit_expr_async(self, node);
        self.deferred(false);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if bare(&node.method) == "try_pop" {
            self.record(PopSite {
                pos: pos(node.method.span().start()),
                kind: PopKind::Method,
            });
        }
        visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        if let Some(last) = node.path.segments.last()
            && bare(&last.ident) == "try_pop"
        {
            self.record(PopSite {
                pos: pos(last.ident.span().start()),
                kind: PopKind::Path,
            });
        }
        visit::visit_expr_path(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        for site in macro_pops(&node.tokens) {
            self.record(site);
        }
    }
}

/// The path of a type for a canonical function path: its segments without generic arguments.
fn type_name(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(path) if path.qself.is_none() => path_name(&path.path),
        syn::Type::Reference(reference) => format!(
            "&{}{}",
            if reference.mutability.is_some() {
                "mut "
            } else {
                ""
            },
            type_name(&reference.elem)
        ),
        syn::Type::Paren(inner) => type_name(&inner.elem),
        syn::Type::Group(inner) => type_name(&inner.elem),
        // A type with no path name (a slice, a tuple, a `dyn` object) has no canonical spelling,
        // so no allowlist entry can name its methods.
        _ => "<type>".to_owned(),
    }
}

fn path_name(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|segment| bare(&segment.ident))
        .collect::<Vec<_>>()
        .join("::")
}

/// The attributes of an item of any kind. A node `syn` keeps as raw tokens has none to read.
fn item_attributes(item: &syn::Item) -> &[syn::Attribute] {
    match item {
        syn::Item::Const(node) => &node.attrs,
        syn::Item::Enum(node) => &node.attrs,
        syn::Item::ExternCrate(node) => &node.attrs,
        syn::Item::Fn(node) => &node.attrs,
        syn::Item::ForeignMod(node) => &node.attrs,
        syn::Item::Impl(node) => &node.attrs,
        syn::Item::Macro(node) => &node.attrs,
        syn::Item::Mod(node) => &node.attrs,
        syn::Item::Static(node) => &node.attrs,
        syn::Item::Struct(node) => &node.attrs,
        syn::Item::Trait(node) => &node.attrs,
        syn::Item::TraitAlias(node) => &node.attrs,
        syn::Item::Type(node) => &node.attrs,
        syn::Item::Union(node) => &node.attrs,
        syn::Item::Use(node) => &node.attrs,
        _ => &[],
    }
}

/// The attributes of an expression of any kind.
fn expr_attributes(expr: &syn::Expr) -> &[syn::Attribute] {
    match expr {
        syn::Expr::Array(node) => &node.attrs,
        syn::Expr::Assign(node) => &node.attrs,
        syn::Expr::Async(node) => &node.attrs,
        syn::Expr::Await(node) => &node.attrs,
        syn::Expr::Binary(node) => &node.attrs,
        syn::Expr::Block(node) => &node.attrs,
        syn::Expr::Break(node) => &node.attrs,
        syn::Expr::Call(node) => &node.attrs,
        syn::Expr::Cast(node) => &node.attrs,
        syn::Expr::Closure(node) => &node.attrs,
        syn::Expr::Const(node) => &node.attrs,
        syn::Expr::Continue(node) => &node.attrs,
        syn::Expr::Field(node) => &node.attrs,
        syn::Expr::ForLoop(node) => &node.attrs,
        syn::Expr::Group(node) => &node.attrs,
        syn::Expr::If(node) => &node.attrs,
        syn::Expr::Index(node) => &node.attrs,
        syn::Expr::Infer(node) => &node.attrs,
        syn::Expr::Let(node) => &node.attrs,
        syn::Expr::Lit(node) => &node.attrs,
        syn::Expr::Loop(node) => &node.attrs,
        syn::Expr::Macro(node) => &node.attrs,
        syn::Expr::Match(node) => &node.attrs,
        syn::Expr::MethodCall(node) => &node.attrs,
        syn::Expr::Paren(node) => &node.attrs,
        syn::Expr::Path(node) => &node.attrs,
        syn::Expr::Range(node) => &node.attrs,
        syn::Expr::RawAddr(node) => &node.attrs,
        syn::Expr::Reference(node) => &node.attrs,
        syn::Expr::Repeat(node) => &node.attrs,
        syn::Expr::Return(node) => &node.attrs,
        syn::Expr::Struct(node) => &node.attrs,
        syn::Expr::Try(node) => &node.attrs,
        syn::Expr::TryBlock(node) => &node.attrs,
        syn::Expr::Tuple(node) => &node.attrs,
        syn::Expr::Unary(node) => &node.attrs,
        syn::Expr::Unsafe(node) => &node.attrs,
        syn::Expr::While(node) => &node.attrs,
        syn::Expr::Yield(node) => &node.attrs,
        // `Verbatim` holds raw tokens; the parser's verbatim finding refuses it.
        _ => &[],
    }
}
