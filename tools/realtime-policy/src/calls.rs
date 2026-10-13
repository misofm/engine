//! The call rule (#1444): inside a marked region, a call to a function that pops, in a loop or in
//! a closure, is refused.
//!
//! The drain rule judges a pop where it stands. A pop in a helper stands in no loop, so a caller
//! that loops over the helper drains the queue until it is empty. This rule closes that: the name
//! set (D1) holds the name of every non-test function under the scan roots whose own body holds a
//! pop (closures count as part of the body, nested `fn` items are their own functions; test code
//! is out; a `feature = ".."` predicate is read as unknown, and an unknown cfg is not test code).
//! The name is matched, not the type: the rule fails closed. Three rules (D2):
//!
//! 1. In a marked region, in a loop (the body or the header of a `for`, `while` or `loop`; a
//!    `for`'s iterator expression is outside it) or in a closure body: a method call whose method is
//!    in the set; a call whose callee path ends in a name of the set; a call whose callee is a
//!    single-segment path that resolves to a `let` whose initializer names a function of the set
//!    anywhere, wherever that `let` stands (`let next = Self::next_record;`, a closure, a cast); and
//!    in a macro token tree an identifier of the set followed by a `( .. )` group.
//! 2. Anywhere in a marked region: a path that is not a callee and ends in a name of the set. A
//!    single-segment path is exempt only when it resolves to a local `let`, a pattern or a
//!    parameter.
//! 3. Anywhere under the roots: an `impl` of `Iterator`, `DoubleEndedIterator` or `IntoIterator`
//!    (or a name that a `use` renames one of them to) with a method that pops. Iteration calls
//!    `next` with no call site.
//!
//! The only fix is a restructure (D3): the pop leaves the loop or the closure, or moves into the
//! loop's own body where the drain rule judges it. Never a rename or a wrapper.
//!
//! Limits that stay (D4): a loop in an unmarked caller (the helper's pop must still be marked or
//! listed, #1443); a helper two calls deep (the loop calls `a`, `a` calls `b`, `b` pops); a function
//! value that enters a function as a parameter, a field or a return value; an implicit call other
//! than iteration (`Drop`, `Deref`, an operator trait).

use std::collections::{BTreeSet, HashMap};

use syn::visit::{self, Visit};

use crate::drain::bare;
use crate::markers::Region;
use crate::report::{Class, Finding};
use crate::source::Source;
use crate::unmarked::{Popping, Unit};

/// The trait names whose `impl` iterates (D2 rule 3).
const ITERATORS: [&str; 3] = ["Iterator", "DoubleEndedIterator", "IntoIterator"];

/// The names that `use` items of `source` rename an iterator trait to, token by token: a rename
/// inside a macro token tree counts, and so does one in a file no rule parses.
pub(crate) fn read_aliases(source: &Source, aliases: &mut BTreeSet<String>) {
    fn scan(tokens: proc_macro2::TokenStream, aliases: &mut BTreeSet<String>) {
        let list: Vec<proc_macro2::TokenTree> = tokens.into_iter().collect();
        for window in list.windows(3) {
            if let [
                proc_macro2::TokenTree::Ident(original),
                proc_macro2::TokenTree::Ident(word),
                proc_macro2::TokenTree::Ident(rename),
            ] = window
                && ITERATORS.contains(&bare(original).as_str())
                && word == "as"
            {
                aliases.insert(bare(rename));
            }
        }
        for token in list {
            if let proc_macro2::TokenTree::Group(group) = token {
                scan(group.stream(), aliases);
            }
        }
    }
    scan(source.tokens.clone(), aliases);
}

/// Judge the files of `units` that hold regions (rules 1 and 2) and every non-test iterator
/// `impl` (rule 3).
pub(crate) fn check(
    units: &[Unit<'_>],
    popping: &Popping,
    aliases: &BTreeSet<String>,
    findings: &mut Vec<Finding>,
) {
    for unit in units.iter().filter(|unit| !unit.regions.is_empty()) {
        let mut calls = Calls {
            names: &popping.names,
            regions: unit.regions,
            scopes: Vec::new(),
            repeats: 0,
            hits: 0,
            found: BTreeSet::new(),
        };
        calls.visit_file(unit.file);
        findings.extend(calls.found.into_iter().map(|(class, line)| {
            Finding::at_line(class, &unit.source.path, line, unit.source.line_text(line))
        }));
    }
    for found in popping.trait_impls.iter().filter(|found| {
        found.popping
            && (ITERATORS.contains(&found.trait_name.as_str())
                || aliases.contains(&found.trait_name))
    }) {
        let source = units[found.unit].source;
        findings.push(Finding::at_line(
            Class::PoppingIterator,
            &source.path,
            found.line,
            source.line_text(found.line),
        ));
    }
}

struct Calls<'a> {
    names: &'a BTreeSet<String>,
    regions: &'a [Region],
    /// The bindings around the node, innermost last: the name and whether it is a local whose
    /// initializer names a function of the set.
    scopes: Vec<HashMap<String, bool>>,
    /// How many loops and closures surround the node, within its function.
    repeats: usize,
    /// How many names of the set (and hot locals) the walk has met, in or out of a region.
    hits: usize,
    found: BTreeSet<(Class, usize)>,
}

impl Calls<'_> {
    fn in_region(&self, line: usize) -> bool {
        self.regions
            .iter()
            .any(|region| region.begin < line && line < region.end)
    }

    /// A call-rule finding for a name at `line`: in a region, in a loop or closure.
    fn call(&mut self, line: usize) {
        if self.repeats > 0 && self.in_region(line) {
            self.found.insert((Class::PoppingCall, line));
        }
    }

    /// The binding of the single-segment name: `Some(hot)` for a local, `None` for an item.
    fn local(&self, name: &str) -> Option<bool> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
    }

    fn bind(&mut self, pattern: &syn::Pat, hot: bool) {
        struct Names(Vec<String>);
        impl<'ast> Visit<'ast> for Names {
            fn visit_pat_ident(&mut self, node: &'ast syn::PatIdent) {
                self.0.push(bare(&node.ident));
                visit::visit_pat_ident(self, node);
            }
        }
        let mut names = Names(Vec::new());
        names.visit_pat(pattern);
        if let Some(scope) = self.scopes.last_mut() {
            scope.extend(names.0.into_iter().map(|name| (name, hot)));
        }
    }

    /// Walk a function: its parameters are bound in a scope of their own.
    fn function(&mut self, signature: &syn::Signature, body: Option<&syn::Block>) {
        self.scopes.push(HashMap::new());
        for input in &signature.inputs {
            if let syn::FnArg::Typed(typed) = input {
                self.bind(&typed.pat, false);
            }
        }
        if let Some(body) = body {
            self.visit_block(body);
        }
        self.scopes.pop();
    }

    /// Meet the identifiers of a macro's tokens.
    fn tokens(&mut self, tokens: &proc_macro2::TokenStream) {
        let list: Vec<proc_macro2::TokenTree> = tokens.clone().into_iter().collect();
        for (at, token) in list.iter().enumerate() {
            match token {
                proc_macro2::TokenTree::Ident(ident) => {
                    let name = bare(ident);
                    let hot = match self.local(&name) {
                        Some(hot) => hot,
                        None => self.names.contains(&name),
                    };
                    if hot {
                        self.hits += 1;
                        if matches!(list.get(at + 1), Some(proc_macro2::TokenTree::Group(group))
                            if group.delimiter() == proc_macro2::Delimiter::Parenthesis)
                        {
                            self.call(ident.span().start().line);
                        }
                    }
                }
                proc_macro2::TokenTree::Group(group) => self.tokens(&group.stream()),
                _ => {}
            }
        }
    }
}

/// The expression inside any parentheses or invisible groups.
fn peel(mut expr: &syn::Expr) -> &syn::Expr {
    loop {
        match expr {
            syn::Expr::Paren(inner) => expr = &inner.expr,
            syn::Expr::Group(inner) => expr = &inner.expr,
            other => return other,
        }
    }
}

impl<'ast> Visit<'ast> for Calls<'_> {
    fn visit_item(&mut self, node: &'ast syn::Item) {
        // An item sees no local of the function around it, and no loop of it.
        let scopes = std::mem::take(&mut self.scopes);
        let repeats = std::mem::take(&mut self.repeats);
        visit::visit_item(self, node);
        self.scopes = scopes;
        self.repeats = repeats;
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.function(&node.sig, Some(&node.block));
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        self.function(&node.sig, Some(&node.block));
    }

    fn visit_trait_item_fn(&mut self, node: &'ast syn::TraitItemFn) {
        self.function(&node.sig, node.default.as_ref());
    }

    fn visit_block(&mut self, node: &'ast syn::Block) {
        self.scopes.push(HashMap::new());
        visit::visit_block(self, node);
        self.scopes.pop();
    }

    fn visit_local(&mut self, node: &'ast syn::Local) {
        let before = self.hits;
        if let Some(init) = &node.init {
            self.visit_expr(&init.expr);
            if let Some((_, diverge)) = &init.diverge {
                self.visit_expr(diverge);
            }
        }
        let hot = self.hits != before;
        self.bind(&node.pat, hot);
    }

    fn visit_expr_let(&mut self, node: &'ast syn::ExprLet) {
        self.visit_expr(&node.expr);
        self.bind(&node.pat, false);
    }

    fn visit_expr_if(&mut self, node: &'ast syn::ExprIf) {
        self.scopes.push(HashMap::new());
        self.visit_expr(&node.cond);
        self.visit_block(&node.then_branch);
        self.scopes.pop();
        if let Some((_, other)) = &node.else_branch {
            self.visit_expr(other);
        }
    }

    fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
        self.visit_expr(&node.expr);
        for arm in &node.arms {
            self.scopes.push(HashMap::new());
            self.bind(&arm.pat, false);
            if let Some((_, guard)) = &arm.guard {
                self.visit_expr(guard);
            }
            self.visit_expr(&arm.body);
            self.scopes.pop();
        }
    }

    fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {
        self.visit_expr(&node.expr);
        self.repeats += 1;
        self.scopes.push(HashMap::new());
        self.bind(&node.pat, false);
        self.visit_block(&node.body);
        self.scopes.pop();
        self.repeats -= 1;
    }

    fn visit_expr_while(&mut self, node: &'ast syn::ExprWhile) {
        self.repeats += 1;
        self.scopes.push(HashMap::new());
        self.visit_expr(&node.cond);
        self.visit_block(&node.body);
        self.scopes.pop();
        self.repeats -= 1;
    }

    fn visit_expr_loop(&mut self, node: &'ast syn::ExprLoop) {
        self.repeats += 1;
        self.visit_block(&node.body);
        self.repeats -= 1;
    }

    fn visit_expr_closure(&mut self, node: &'ast syn::ExprClosure) {
        self.repeats += 1;
        self.scopes.push(HashMap::new());
        for input in &node.inputs {
            self.bind(input, false);
        }
        self.visit_expr(&node.body);
        self.scopes.pop();
        self.repeats -= 1;
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let syn::Expr::Path(callee) = peel(&node.func)
            && let Some(last) = callee.path.segments.last()
        {
            let name = bare(&last.ident);
            let single = callee.qself.is_none()
                && callee.path.leading_colon.is_none()
                && callee.path.segments.len() == 1;
            let hot = match single.then(|| self.local(&name)).flatten() {
                Some(hot) => hot,
                None => self.names.contains(&name),
            };
            if hot {
                self.hits += 1;
                self.call(last.ident.span().start().line);
            }
        } else {
            self.visit_expr(&node.func);
        }
        for argument in &node.args {
            self.visit_expr(argument);
        }
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if self.names.contains(&bare(&node.method)) {
            self.hits += 1;
            self.call(node.method.span().start().line);
        }
        visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        let Some(last) = node.path.segments.last() else {
            return;
        };
        let name = bare(&last.ident);
        let single = node.qself.is_none()
            && node.path.leading_colon.is_none()
            && node.path.segments.len() == 1;
        match single.then(|| self.local(&name)).flatten() {
            // A local `let`, pattern or parameter: exempt as a value, but a hot one carries the
            // function into whatever it initializes.
            Some(hot) => self.hits += usize::from(hot),
            None => {
                if self.names.contains(&name) {
                    self.hits += 1;
                    let line = last.ident.span().start().line;
                    if self.in_region(line) {
                        self.found.insert((Class::PoppingPathValue, line));
                    }
                }
            }
        }
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        self.tokens(&node.tokens);
    }
}
