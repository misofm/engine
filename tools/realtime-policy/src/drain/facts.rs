//! What one visit of a parsed file records: its pops, its uses of local names, its parsed
//! `debug_assert!` arguments, and where its macro invocations and attributes stand.

use std::collections::HashMap;

use syn::parse::Parser;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};

use super::names::bare;
use super::tree::{Pos, is_debug_assert, macro_key, pos};

/// The one name that a pop spells (D5).
pub(super) const POP: &str = "try_pop";

/// How a pop is spelled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PopKind {
    /// A method call named `try_pop`.
    Method,
    /// A path expression whose last segment is `try_pop`.
    Path,
    /// The identifier `try_pop` inside a macro's token tree.
    MacroToken,
}

/// A pop and the position of its `try_pop` identifier.
#[derive(Clone, Copy, Debug)]
pub(super) struct PopSite {
    pub(super) pos: Pos,
    pub(super) kind: PopKind,
}

/// Every pop of `file`, in no particular order.
pub(super) fn pops(file: &syn::File) -> Vec<PopSite> {
    let mut finder = PopFinder { found: Vec::new() };
    finder.visit_file(file);
    finder.found
}

struct PopFinder {
    found: Vec<PopSite>,
}

impl PopFinder {
    fn tokens(&mut self, tokens: &proc_macro2::TokenStream) {
        for token in tokens.clone() {
            match token {
                proc_macro2::TokenTree::Ident(ident) if bare(&ident) == POP => {
                    self.found.push(PopSite {
                        pos: pos(ident.span().start()),
                        kind: PopKind::MacroToken,
                    });
                }
                proc_macro2::TokenTree::Group(group) => self.tokens(&group.stream()),
                _ => {}
            }
        }
    }
}

impl<'ast> Visit<'ast> for PopFinder {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if bare(&node.method) == POP {
            self.found.push(PopSite {
                pos: pos(node.method.span().start()),
                kind: PopKind::Method,
            });
        }
        visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        if let Some(last) = node.path.segments.last()
            && bare(&last.ident) == POP
        {
            self.found.push(PopSite {
                pos: pos(last.ident.span().start()),
                kind: PopKind::Path,
            });
        }
        visit::visit_expr_path(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        self.tokens(&node.tokens);
    }
}

/// How a name is used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum UseKind {
    /// A single-segment path expression.
    Path,
    /// An identifier in a macro's token tree, where no syntax is parsed.
    Macro,
}

/// A place where a name could be read or changed.
#[derive(Clone, Copy, Debug)]
pub(super) struct Use {
    pub(super) pos: Pos,
    pub(super) kind: UseKind,
}

/// The uses of every name, the parsed `debug_assert!` arguments, and the macro invocations and
/// attributes of one file.
#[derive(Default)]
pub(super) struct Facts {
    /// Whether `debug_assert!` is the standard macro (D2): its arguments are parsed as
    /// expressions. When a file under the scan roots shadows the name, it is any other macro.
    trust_debug_assert: bool,
    /// The uses of each name (a raw identifier is its bare name), sorted by position.
    pub(super) uses: HashMap<String, Vec<Use>>,
    /// The arguments of each `debug_assert!`, parsed as expressions, by [`macro_key`].
    pub(super) debug_args: HashMap<Pos, Vec<syn::Expr>>,
    /// Where each macro invocation starts, by any path and in any position (expression,
    /// statement, item, pattern or type), sorted by position.
    macros: Vec<Pos>,
    /// Where each attribute, outer or inner, starts, sorted by position.
    attributes: Vec<Pos>,
    /// Where each macro invocation in statement or item position starts (D10), sorted by
    /// position. A macro there can expand to an item, which shadows a binding for its whole block.
    /// A `debug_assert!` is left out while it is trusted: it expands to an `if` expression.
    item_macros: Vec<Pos>,
    /// Where each item that carries an attribute other than `cfg`, a lint level or `doc` starts
    /// its first such attribute (D10), sorted by position. A derive or an attribute macro emits
    /// items beside its item.
    attributed_items: Vec<Pos>,
}

impl Facts {
    pub(super) fn of(file: &syn::File, trust_debug_assert: bool) -> Self {
        let mut facts = Self {
            trust_debug_assert,
            ..Self::default()
        };
        facts.visit_file(file);
        for uses in facts.uses.values_mut() {
            uses.sort_by_key(|used| used.pos);
        }
        facts.macros.sort_unstable();
        facts.attributes.sort_unstable();
        facts.item_macros.sort_unstable();
        facts.attributed_items.sort_unstable();
        facts
    }

    /// D10: whether a macro in statement or item position, or an item with an attribute that can
    /// emit items, starts in `from..to`.
    pub(super) fn declares_items_in(&self, from: Pos, to: Pos) -> bool {
        let any = |list: &[Pos]| {
            let first = list.partition_point(|at| *at < from);
            list.get(first).is_some_and(|at| *at < to)
        };
        any(&self.item_macros) || any(&self.attributed_items)
    }

    /// Whether a macro invocation or an attribute starts in `from..to`.
    pub(super) fn has_macro_or_attribute_in(&self, from: Pos, to: Pos) -> bool {
        let any = |list: &[Pos]| {
            let first = list.partition_point(|at| *at < from);
            list.get(first).is_some_and(|at| *at < to)
        };
        any(&self.macros) || any(&self.attributes)
    }

    /// The uses of `name` that start in `from..to`.
    pub(super) fn uses_in(&self, name: &str, from: Pos, to: Pos) -> &[Use] {
        let Some(uses) = self.uses.get(name) else {
            return &[];
        };
        let first = uses.partition_point(|used| used.pos < from);
        let last = uses.partition_point(|used| used.pos < to);
        &uses[first..last.max(first)]
    }

    fn record(&mut self, name: String, at: Pos, kind: UseKind) {
        self.uses
            .entry(name)
            .or_default()
            .push(Use { pos: at, kind });
    }

    /// D10: note a macro in statement or item position, unless it is a trusted `debug_assert!`.
    fn note_item_macro(&mut self, call: &syn::Macro) {
        if !(self.trust_debug_assert && is_debug_assert(call)) {
            self.item_macros.push(pos(call.path.span().start()));
        }
    }

    fn tokens(&mut self, tokens: &proc_macro2::TokenStream) {
        for token in tokens.clone() {
            match token {
                proc_macro2::TokenTree::Ident(ident) => {
                    self.record(bare(&ident), pos(ident.span().start()), UseKind::Macro);
                }
                proc_macro2::TokenTree::Group(group) => self.tokens(&group.stream()),
                _ => {}
            }
        }
    }
}

impl<'ast> Visit<'ast> for Facts {
    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        if node.qself.is_none()
            && node.path.leading_colon.is_none()
            && node.path.segments.len() == 1
            && let Some(segment) = node.path.segments.first()
            && segment.arguments.is_none()
        {
            self.record(
                bare(&segment.ident),
                pos(segment.ident.span().start()),
                UseKind::Path,
            );
        }
        visit::visit_expr_path(self, node);
    }

    fn visit_attribute(&mut self, node: &'ast syn::Attribute) {
        self.attributes.push(pos(node.pound_token.span.start()));
        visit::visit_attribute(self, node);
    }

    fn visit_stmt_macro(&mut self, node: &'ast syn::StmtMacro) {
        self.note_item_macro(&node.mac);
        visit::visit_stmt_macro(self, node);
    }

    fn visit_item_macro(&mut self, node: &'ast syn::ItemMacro) {
        self.note_item_macro(&node.mac);
        visit::visit_item_macro(self, node);
    }

    fn visit_item(&mut self, node: &'ast syn::Item) {
        if let Some(attribute) = item_attributes(node)
            .iter()
            .find(|attribute| !cannot_emit_items(attribute))
        {
            self.attributed_items
                .push(pos(attribute.pound_token.span.start()));
        }
        visit::visit_item(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        self.macros.push(pos(node.path.span().start()));
        let parsed = (self.trust_debug_assert && is_debug_assert(node))
            .then(|| {
                syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated
                    .parse2(node.tokens.clone())
                    .ok()
            })
            .flatten();
        let Some(args) = parsed else {
            self.tokens(&node.tokens);
            return;
        };
        let args: Vec<syn::Expr> = args.into_iter().collect();
        let mut inner = Self {
            trust_debug_assert: true,
            ..Self::default()
        };
        for arg in &args {
            inner.visit_expr(arg);
        }
        for (name, uses) in inner.uses {
            self.uses.entry(name).or_default().extend(uses);
        }
        self.debug_args.extend(inner.debug_args);
        self.macros.extend(inner.macros);
        self.attributes.extend(inner.attributes);
        self.item_macros.extend(inner.item_macros);
        self.attributed_items.extend(inner.attributed_items);
        self.debug_args.insert(macro_key(node), args);
    }
}

/// D10: whether the attribute cannot emit an item: `cfg`, a lint level (`allow`, `expect`, `warn`,
/// `deny`, `forbid`) or `doc`, spelled as one bare name. A derive, an attribute macro and
/// `cfg_attr` (which can expand to a derive) can.
fn cannot_emit_items(attribute: &syn::Attribute) -> bool {
    attribute.path().get_ident().is_some_and(|name| {
        matches!(
            bare(name).as_str(),
            "cfg" | "allow" | "expect" | "warn" | "deny" | "forbid" | "doc"
        )
    })
}

/// The outer attributes of an item of any kind. A node `syn` keeps as raw tokens has none to read;
/// the parser's verbatim finding refuses it.
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
