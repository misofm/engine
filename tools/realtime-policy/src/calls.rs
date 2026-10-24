//! The call rule (#1444): inside a marked region, a call to a function that pops, in a loop or in
//! a closure, is refused.
//!
//! The drain rule judges a pop where it stands. A pop in a helper stands in no loop, so a caller
//! that loops over the helper drains the queue until it is empty. This rule closes that: the name
//! set (D1) holds the name of every non-test function under the scan roots whose own body holds a
//! pop (closures count as part of the body, nested `fn` items are their own functions; test code
//! is out; a `feature = ".."` predicate is read as unknown, and an unknown cfg is not test code).
//! The set also holds (#1544 D2), to a fixpoint, the name of every `const` or `static` item whose
//! initializer names a member, and the new name of every `use` rename (`member as name`): both are
//! read token by token in every file, test code and macro token trees included.
//! The name is matched, not the type: the rule fails closed. Three rules (D2):
//!
//! 1. In a marked region, in a loop (the body or the header of a `for`, `while` or `loop`; a
//!    `for`'s iterator expression is outside it) or in a closure body: a method call whose method is
//!    in the set; a call whose callee path ends in a name of the set; and a call whose callee is a
//!    single-segment path that resolves to a hot local. A local is hot (#1544 D1) when its `let`
//!    initializer, any assignment to it (`x = ..`, `x op= ..`, one written in a macro token tree
//!    included) or the value that a pattern binding it matches (`if let`, `while let`, `match`,
//!    `for`) names a function of the set or a hot local anywhere (`Self::next_record`, a closure,
//!    a cast). The check is flow-insensitive: it reads every `let` and assignment of the function,
//!    wherever it stands, and a deferred `let next;` counts.
//! 2. Anywhere in a marked region: a path that is not a callee and ends in a name of the set. A
//!    single-segment path is exempt only when it resolves to a local `let`, a pattern or a
//!    parameter. In a macro token tree (#1544 D3) an identifier of the set that does not resolve to
//!    a local is refused anywhere in a region, in or out of a loop: as a call when a `( .. )` group
//!    follows it, as a path value otherwise, so a loop or a closure written in the tokens is
//!    covered without parsing them. A hot local followed by `( .. )` in a token tree is refused
//!    anywhere in a region for the same reason. A field name (`.name` with no `( .. )` after it, or
//!    `name:` in a struct literal) is not a value.
//! 3. Anywhere under the roots: an `impl` of `Iterator`, `DoubleEndedIterator` or `IntoIterator`
//!    (or a name that a `use` renames one of them to) with a method that pops. Iteration calls
//!    `next` with no call site.
//!
//! The only fix is a restructure (D3): the pop leaves the loop or the closure, or moves into the
//! loop's own body where the drain rule judges it. Never a rename or a wrapper.
//!
//! Limits that stay (D4): a loop in an unmarked caller (the helper's pop must still be marked or
//! listed, #1443); a helper two calls deep (the loop calls `a`, `a` calls `b`, `b` pops); a function
//! value that enters a function as a parameter, a field or a return value, or that aliasing through
//! `&mut` writes (`*r = ..`, `mem::swap`, `mem::replace`); an implicit call other
//! than iteration (`Drop`, `Deref`, an operator trait); a standard-library adapter that holds a
//! closure taken above the region (`let it = std::iter::from_fn(|| lane.next_record());`, then
//! `for r in it`); a name that a macro builds from parts (`paste!`, a `$f` fragment spliced into a
//! call), limit 4 of the #1438 threat model.

use std::collections::{BTreeSet, HashMap};

use syn::visit::{self, Visit};

use crate::drain::bare;
use crate::markers::Region;
use crate::report::{Class, Finding};
use crate::source::Source;
use crate::unmarked::{Popping, Unit};

/// The trait names whose `impl` iterates (D2 rule 3).
const ITERATORS: [&str; 3] = ["Iterator", "DoubleEndedIterator", "IntoIterator"];

/// A name fact that [`read_aliases`] reads token by token in every file.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Alias {
    /// A name that a `use` renames an iterator trait to (D2 rule 3).
    Iterator(String),
    /// A name that holds a function of the set when any of `sources` is in the set (#1544 D2):
    /// a `const` or `static` item with the identifiers of its initializer, or the new name of a
    /// `use` rename with its original name.
    Value {
        name: String,
        sources: BTreeSet<String>,
    },
}

/// The name facts of `source`, token by token: a fact inside a macro token tree counts, and so
/// does one in a file no rule parses, test code included (the set only grows, so the rule fails
/// closed).
pub(crate) fn read_aliases(source: &Source, aliases: &mut BTreeSet<Alias>) {
    fn punct(token: Option<&proc_macro2::TokenTree>, char: char) -> bool {
        matches!(token, Some(proc_macro2::TokenTree::Punct(punct)) if punct.as_char() == char)
    }
    fn scan(tokens: proc_macro2::TokenStream, aliases: &mut BTreeSet<Alias>) {
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
                aliases.insert(Alias::Iterator(bare(rename)));
            }
        }
        for (at, token) in list.iter().enumerate() {
            let proc_macro2::TokenTree::Ident(word) = token else {
                continue;
            };
            // The tokens up to the `;` that ends the item.
            let statement = || {
                list[at + 1..]
                    .iter()
                    .take_while(|token| !punct(Some(token), ';'))
            };
            if word == "use" {
                renames(statement(), aliases);
            } else if (word == "const" || word == "static")
                // Not a const generic parameter or a `'static` lifetime.
                && !(at > 0 && ['<', ',', '\''].iter().any(|&char| punct(list.get(at - 1), char)))
            {
                let mut rest = statement().peekable();
                if matches!(rest.peek(), Some(proc_macro2::TokenTree::Ident(word)) if word == "mut")
                {
                    rest.next();
                }
                if let Some(proc_macro2::TokenTree::Ident(name)) = rest.next()
                    && punct(rest.next(), ':')
                {
                    let mut initializer = rest.skip_while(|token| {
                        !matches!(token, proc_macro2::TokenTree::Punct(punct)
                            if punct.as_char() == '=' && punct.spacing() == proc_macro2::Spacing::Alone)
                    });
                    let name = bare(name);
                    if initializer.next().is_some() && name != "_" {
                        let initializer: Vec<proc_macro2::TokenTree> =
                            initializer.cloned().collect();
                        let mut sources = BTreeSet::new();
                        values(&initializer, &mut sources);
                        aliases.insert(Alias::Value { name, sources });
                    }
                }
            }
        }
        for token in list {
            if let proc_macro2::TokenTree::Group(group) = token {
                scan(group.stream(), aliases);
            }
        }
    }
    /// Every `original as rename` of a `use` tree, at any depth.
    fn renames<'t>(
        tokens: impl Iterator<Item = &'t proc_macro2::TokenTree>,
        aliases: &mut BTreeSet<Alias>,
    ) {
        let list: Vec<&proc_macro2::TokenTree> = tokens.collect();
        for window in list.windows(3) {
            if let [
                proc_macro2::TokenTree::Ident(original),
                proc_macro2::TokenTree::Ident(word),
                proc_macro2::TokenTree::Ident(rename),
            ] = window
                && word == "as"
            {
                aliases.insert(Alias::Value {
                    name: bare(rename),
                    sources: BTreeSet::from([bare(original)]),
                });
            }
        }
        for token in list {
            if let proc_macro2::TokenTree::Group(group) = token {
                let stream: Vec<proc_macro2::TokenTree> = group.stream().into_iter().collect();
                renames(stream.iter(), aliases);
            }
        }
    }
    scan(source.tokens.clone(), aliases);
}

/// Whether the identifier at `at` is a field name: of a struct literal (`meters: true`), or of a
/// field read (`.meters` with no `( .. )` or `::<` after it).
fn field(tokens: &[proc_macro2::TokenTree], at: usize) -> bool {
    let next = tokens.get(at + 1);
    let literal = matches!(next, Some(proc_macro2::TokenTree::Punct(punct))
        if punct.as_char() == ':' && punct.spacing() == proc_macro2::Spacing::Alone);
    let read = at > 0
        && matches!(&tokens[at - 1], proc_macro2::TokenTree::Punct(punct) if punct.as_char() == '.')
        && !matches!(next, Some(proc_macro2::TokenTree::Group(group))
            if group.delimiter() == proc_macro2::Delimiter::Parenthesis)
        && !matches!(next, Some(proc_macro2::TokenTree::Punct(punct)) if punct.as_char() == ':');
    literal || read
}

/// Every identifier of `tokens` at any depth that can name a function: not a field name
/// ([`field`]) and not a module segment (`next_record::DEPTH`).
fn values(tokens: &[proc_macro2::TokenTree], found: &mut BTreeSet<String>) {
    for (at, token) in tokens.iter().enumerate() {
        match token {
            proc_macro2::TokenTree::Ident(ident) => {
                // A module segment: `name::item`, not the turbofish `name::<T>`.
                let module = matches!(
                    tokens.get(at + 1..at + 4),
                    Some([
                        proc_macro2::TokenTree::Punct(first),
                        proc_macro2::TokenTree::Punct(second),
                        proc_macro2::TokenTree::Ident(_),
                    ]) if first.as_char() == ':'
                        && first.spacing() == proc_macro2::Spacing::Joint
                        && second.as_char() == ':'
                );
                if !(field(tokens, at) || module) {
                    found.insert(bare(ident));
                }
            }
            proc_macro2::TokenTree::Group(group) => {
                let inner: Vec<proc_macro2::TokenTree> = group.stream().into_iter().collect();
                values(&inner, found);
            }
            _ => {}
        }
    }
}

/// Every identifier of `token` at any depth.
fn idents(token: &proc_macro2::TokenTree, found: &mut BTreeSet<String>) {
    match token {
        proc_macro2::TokenTree::Ident(ident) => {
            found.insert(bare(ident));
        }
        proc_macro2::TokenTree::Group(group) => {
            for inner in group.stream() {
                idents(&inner, found);
            }
        }
        _ => {}
    }
}

/// The set of #1444 D1 with every item and rename of #1544 D2 that reaches it, to a fixpoint.
fn names(popping: &Popping, aliases: &BTreeSet<Alias>) -> BTreeSet<String> {
    let mut names = popping.names.clone();
    let values: Vec<(&String, &BTreeSet<String>)> = aliases
        .iter()
        .filter_map(|alias| match alias {
            Alias::Value { name, sources } => Some((name, sources)),
            Alias::Iterator(_) => None,
        })
        .collect();
    loop {
        let before = names.len();
        for (name, sources) in &values {
            if !names.contains(*name) && sources.iter().any(|source| names.contains(source)) {
                names.insert((*name).clone());
            }
        }
        if names.len() == before {
            return names;
        }
    }
}

/// Judge the files of `units` that hold regions (rules 1 and 2) and every non-test iterator
/// `impl` (rule 3).
pub(crate) fn check(
    units: &[Unit<'_>],
    popping: &Popping,
    aliases: &BTreeSet<Alias>,
    findings: &mut Vec<Finding>,
) {
    let names = names(popping, aliases);
    for unit in units.iter().filter(|unit| !unit.regions.is_empty()) {
        let mut calls = Calls {
            names: &names,
            regions: unit.regions,
            scopes: Vec::new(),
            assigned: BTreeSet::new(),
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
                || aliases.contains(&Alias::Iterator(found.trait_name.clone())))
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
    /// The locals of the function around the node that an assignment or a `let` makes hot,
    /// wherever it stands (#1544 D1).
    assigned: BTreeSet<String>,
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
            let assigned = &self.assigned;
            scope.extend(names.0.into_iter().map(|name| {
                let hot = hot || assigned.contains(&name);
                (name, hot)
            }));
        }
    }

    /// Walk a function: its parameters are bound in a scope of their own.
    fn function(&mut self, signature: &syn::Signature, body: Option<&syn::Block>) {
        let outer = std::mem::replace(&mut self.assigned, assigned(self.names, body));
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
        self.assigned = outer;
    }

    /// Meet the identifiers of a macro's tokens (#1544 D3): in a region, a name of the set is
    /// refused anywhere, and a hot local wherever it is called. A field name is not a value.
    fn tokens(&mut self, tokens: &proc_macro2::TokenStream) {
        let list: Vec<proc_macro2::TokenTree> = tokens.clone().into_iter().collect();
        for (at, token) in list.iter().enumerate() {
            match token {
                proc_macro2::TokenTree::Ident(ident) => {
                    let name = bare(ident);
                    let called = matches!(list.get(at + 1), Some(proc_macro2::TokenTree::Group(group))
                        if group.delimiter() == proc_macro2::Delimiter::Parenthesis);
                    let line = ident.span().start().line;
                    if field(&list, at) {
                        continue;
                    }
                    match self.local(&name) {
                        // A loop or a closure written in the tokens is not seen, so a call of a
                        // hot local is refused anywhere in a region.
                        Some(true) => {
                            self.hits += 1;
                            if called && self.in_region(line) {
                                self.found.insert((Class::PoppingCall, line));
                            }
                        }
                        Some(false) => {}
                        None if self.names.contains(&name) => {
                            self.hits += 1;
                            if self.in_region(line) {
                                let class = if called {
                                    Class::PoppingCall
                                } else {
                                    Class::PoppingPathValue
                                };
                                self.found.insert((class, line));
                            }
                        }
                        None => {}
                    }
                }
                proc_macro2::TokenTree::Group(group) => self.tokens(&group.stream()),
                _ => {}
            }
        }
    }
}

/// The locals of a function body that a name of the set or another such local reaches through a
/// `let` initializer, an assignment or a pattern of an `if let`, `while let`, `match` or `for`, to a
/// fixpoint (#1544 D1). Flow-insensitive: every `let` and
/// assignment of the body counts wherever it stands, closures included; a nested item is its own.
fn assigned(names: &BTreeSet<String>, body: Option<&syn::Block>) -> BTreeSet<String> {
    /// Each edge: the names it binds or assigns, and the identifiers of its value.
    #[derive(Default)]
    struct Edges(Vec<(BTreeSet<String>, BTreeSet<String>)>);
    /// The identifiers of a node, a macro's tokens included.
    #[derive(Default)]
    struct Idents(BTreeSet<String>);
    impl<'ast> Visit<'ast> for Idents {
        fn visit_ident(&mut self, node: &'ast proc_macro2::Ident) {
            self.0.insert(bare(node));
        }
        fn visit_macro(&mut self, node: &'ast syn::Macro) {
            visit::visit_macro(self, node);
            for token in node.tokens.clone() {
                idents(&token, &mut self.0);
            }
        }
    }
    /// The single-segment paths of an assignee expression, or the names a pattern binds.
    #[derive(Default)]
    struct Targets(BTreeSet<String>);
    impl<'ast> Visit<'ast> for Targets {
        fn visit_pat_ident(&mut self, node: &'ast syn::PatIdent) {
            self.0.insert(bare(&node.ident));
            visit::visit_pat_ident(self, node);
        }
        fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
            if node.qself.is_none()
                && let Some(ident) = node.path.get_ident()
            {
                self.0.insert(bare(ident));
            }
        }
    }
    impl Edges {
        /// The names a pattern binds, from the identifiers of the value it matches.
        fn bind(&mut self, pattern: &syn::Pat, value: &syn::Expr) {
            let mut targets = Targets::default();
            targets.visit_pat(pattern);
            let mut sources = Idents::default();
            sources.visit_expr(value);
            self.0.push((targets.0, sources.0));
        }
        fn assign(&mut self, left: &syn::Expr, right: &syn::Expr) {
            let mut targets = Targets::default();
            targets.visit_expr(left);
            let mut sources = Idents::default();
            sources.visit_expr(right);
            self.0.push((targets.0, sources.0));
        }
    }
    impl<'ast> Visit<'ast> for Edges {
        fn visit_item(&mut self, _: &'ast syn::Item) {}
        fn visit_local(&mut self, node: &'ast syn::Local) {
            if let Some(init) = &node.init {
                let mut targets = Targets::default();
                targets.visit_pat(&node.pat);
                let mut sources = Idents::default();
                sources.visit_local_init(init);
                self.0.push((targets.0, sources.0));
            }
            visit::visit_local(self, node);
        }
        fn visit_expr_let(&mut self, node: &'ast syn::ExprLet) {
            self.bind(&node.pat, &node.expr);
            visit::visit_expr_let(self, node);
        }
        fn visit_expr_match(&mut self, node: &'ast syn::ExprMatch) {
            for arm in &node.arms {
                self.bind(&arm.pat, &node.expr);
            }
            visit::visit_expr_match(self, node);
        }
        fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {
            self.bind(&node.pat, &node.expr);
            visit::visit_expr_for_loop(self, node);
        }
        fn visit_expr_assign(&mut self, node: &'ast syn::ExprAssign) {
            self.assign(&node.left, &node.right);
            visit::visit_expr_assign(self, node);
        }
        fn visit_expr_binary(&mut self, node: &'ast syn::ExprBinary) {
            if matches!(
                node.op,
                syn::BinOp::AddAssign(_)
                    | syn::BinOp::SubAssign(_)
                    | syn::BinOp::MulAssign(_)
                    | syn::BinOp::DivAssign(_)
                    | syn::BinOp::RemAssign(_)
                    | syn::BinOp::BitXorAssign(_)
                    | syn::BinOp::BitAndAssign(_)
                    | syn::BinOp::BitOrAssign(_)
                    | syn::BinOp::ShlAssign(_)
                    | syn::BinOp::ShrAssign(_)
            ) {
                self.assign(&node.left, &node.right);
            }
            visit::visit_expr_binary(self, node);
        }
        fn visit_macro(&mut self, node: &'ast syn::Macro) {
            // A token tree: every identifier before an `=` or an `op=` (not `==`, `!=`, `<=` or
            // `>=`) is assigned from the identifiers of the whole tree.
            fn targets(tokens: proc_macro2::TokenStream, found: &mut BTreeSet<String>) {
                let list: Vec<proc_macro2::TokenTree> = tokens.into_iter().collect();
                for (at, token) in list.iter().enumerate() {
                    match token {
                        proc_macro2::TokenTree::Ident(ident) => {
                            // The operator after the identifier: its punctuation up to the first
                            // one that stands alone.
                            let mut operator = String::new();
                            for token in &list[at + 1..] {
                                let proc_macro2::TokenTree::Punct(punct) = token else {
                                    break;
                                };
                                operator.push(punct.as_char());
                                if punct.spacing() == proc_macro2::Spacing::Alone {
                                    break;
                                }
                            }
                            let assigns = operator.ends_with('=')
                                && !matches!(operator.as_str(), "==" | "!=" | "<=" | ">=");
                            if assigns {
                                found.insert(bare(ident));
                            }
                        }
                        proc_macro2::TokenTree::Group(group) => targets(group.stream(), found),
                        _ => {}
                    }
                }
            }
            let mut assigned = BTreeSet::new();
            targets(node.tokens.clone(), &mut assigned);
            let mut sources = BTreeSet::new();
            for token in node.tokens.clone() {
                idents(&token, &mut sources);
            }
            self.0.push((assigned, sources));
        }
    }
    let mut edges = Edges::default();
    if let Some(body) = body {
        edges.visit_block(body);
    }
    let mut hot = BTreeSet::new();
    loop {
        let before = hot.len();
        for (targets, sources) in &edges.0 {
            if sources
                .iter()
                .any(|source| names.contains(source) || hot.contains(source))
            {
                hot.extend(targets.iter().cloned());
            }
        }
        if hot.len() == before {
            return hot;
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
        let assigned = std::mem::take(&mut self.assigned);
        let repeats = std::mem::take(&mut self.repeats);
        visit::visit_item(self, node);
        self.scopes = scopes;
        self.assigned = assigned;
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
