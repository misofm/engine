//! The render-locked wrap rule: every wrapped export of the render-thread table is one call.
//!
//! Decision 15 D15-10 and issue #1488: each export that either AudioWorklet calls after
//! construction runs inside `render_locked`, so the browser build's allocator counts what it
//! allocates. No allocation count can defend the wrap itself: without the wrap there is no window,
//! and the count reads zero. The syntax tree can. For each wrapped entry of the policy's table the
//! export must exist as a top-level `#[unsafe(no_mangle)]` function of `hosts/host-web/src/ffi.rs`
//! whose body is exactly one statement, a call whose callee is `render_locked`, resolved to
//! `crate::render_lock::render_locked`, with one closure as its only argument. `render_locked` is
//! resolved when the file imports it from `crate::render_lock` and defines no other name
//! `render_locked` anywhere (a function, a macro, any item, a `use` of another path or a binding),
//! or when the callee is written as the full path.
//!
//! The export's symbol is the listed function's own only if nothing else can take the name
//! (#1528 D1): a listed export with a generic parameter, a `where` clause or an `impl Trait` in
//! its signature emits no symbol and is refused, and so is a `no_mangle` function named like a
//! listed export anywhere under `hosts/host-web/src` other than the top level of `ffi.rs`
//! ([`scan_twins`]).
//!
//! The syntax tree of one file proves the wrap only if the build compiles that file as read.
//! `web_host` refuses what could make it not (conditional compilation on the import, an export, a
//! wrap call or a guarded module, `#[export_name]`, `#[path]` and glob `use`); a cfg-split twin
//! or a glob pass-through is refused there, not resolved here.

use syn::visit::{self, Visit};

use crate::policy::Policy;
use crate::report::{Class, Finding};
use crate::source::Source;
use crate::web_host::{Parsed, mentions, refuse_conditional};

/// The file the rule reads, relative to the workspace root.
pub(crate) const WEB_FFI: &str = "hosts/host-web/src/ffi.rs";

/// The function every wrapped export calls.
const RENDER_LOCKED: &str = "render_locked";

/// Check `file`, which is `WEB_FFI` parsed from `source`.
pub(crate) fn scan(
    source: &Source,
    file: &syn::File,
    policy: &Policy,
    findings: &mut Vec<Finding>,
) {
    let resolution = Resolution::of(file);
    for line in &resolution.conditional_imports {
        findings.push(Finding::at_line(
            Class::ConditionalWrapPath,
            &source.path,
            *line,
            source.line_text(*line),
        ));
    }
    if !resolution.imported {
        findings.push(Finding::at_path(
            Class::RenderLockedNotImported,
            &source.path,
        ));
    }
    for line in &resolution.shadows {
        findings.push(Finding::at_line(
            Class::RenderLockedShadowed,
            &source.path,
            *line,
            source.line_text(*line),
        ));
    }
    let resolved = resolution.imported && resolution.shadows.is_empty();
    let exports: Vec<&syn::ItemFn> = file
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Fn(function) if is_no_mangle(function) => Some(function),
            _ => None,
        })
        .collect();
    for entry in &policy.render_thread_exports {
        let found: Vec<&&syn::ItemFn> = exports
            .iter()
            .filter(|function| function.sig.ident == entry.name.as_str())
            .collect();
        for function in &found {
            refuse_conditional(source, &function.attrs, Some(&entry.name), findings);
            if is_generic(&function.sig) {
                let line = function.sig.ident.span().start().line;
                findings.push(
                    Finding::at_line(
                        Class::GenericWebExport,
                        &source.path,
                        line,
                        source.line_text(line),
                    )
                    .about(&entry.name),
                );
            }
        }
        let [function] = found.as_slice() else {
            let class = if found.is_empty() {
                Class::ExportMissing
            } else {
                Class::ExportDefinedTwice
            };
            findings.push(Finding::at_path(class, &source.path).about(&entry.name));
            continue;
        };
        let line = function.sig.ident.span().start().line;
        let at = |class| Finding::at_line(class, &source.path, line, source.line_text(line));
        if !entry.wrapped {
            if !policy.unwrapped_exports.contains(&entry.name) {
                findings.push(at(Class::UnwrappedNotPermitted).about(&entry.name));
            }
            continue;
        }
        if let Some(call) = sole_call(function) {
            refuse_conditional(source, &call.attrs, Some(&entry.name), findings);
        }
        match callee_of_sole_call(function) {
            None => findings.push(at(Class::RenderLockedBody).about(&entry.name)),
            Some(Callee::Bare) if !resolved => {
                findings.push(at(Class::RenderLockedUnresolved).about(&entry.name));
            }
            Some(Callee::Bare | Callee::FullPath) => {}
            Some(Callee::Other) => findings.push(at(Class::RenderLockedBody).about(&entry.name)),
        }
    }
}

/// D1(a): whether the signature has a generic parameter of any kind, a `where` clause, or an
/// `impl Trait` anywhere in an argument or the return type. rustc emits no symbol for such a
/// function, `#[no_mangle]` or not.
fn is_generic(signature: &syn::Signature) -> bool {
    struct ImplTrait(bool);
    impl<'ast> Visit<'ast> for ImplTrait {
        fn visit_type_impl_trait(&mut self, _: &'ast syn::TypeImplTrait) {
            self.0 = true;
        }
    }
    if !signature.generics.params.is_empty() || signature.generics.where_clause.is_some() {
        return true;
    }
    let mut finder = ImplTrait(false);
    finder.visit_signature(signature);
    finder.0
}

/// D1(c): every function or static named like a listed export that carries `no_mangle` in any
/// spelling (an attribute with the identifier `no_mangle` among its tokens: `#[no_mangle]`,
/// `#[unsafe(no_mangle)]`, inside `cfg_attr`), in any module, `impl` or trait block or function
/// body of `files`, except the top-level functions of `ffi.rs`, which [`scan`] judges.
pub(crate) fn scan_twins(files: &[Parsed<'_>], policy: &Policy, findings: &mut Vec<Finding>) {
    let names: Vec<&str> = policy
        .render_thread_exports
        .iter()
        .map(|entry| entry.name.as_str())
        .collect();
    for (source, file) in files {
        let mut twins = Twins {
            source,
            names: &names,
            findings: &mut *findings,
        };
        if source.path == WEB_FFI {
            for item in &file.items {
                match item {
                    // A top-level function of `ffi.rs` is the export itself; what it holds is not.
                    syn::Item::Fn(function) => visit::visit_item_fn(&mut twins, function),
                    other => twins.visit_item(other),
                }
            }
        } else {
            twins.visit_file(file);
        }
    }
}

struct Twins<'a> {
    source: &'a Source,
    names: &'a [&'a str],
    findings: &'a mut Vec<Finding>,
}

impl Twins<'_> {
    fn judge(&mut self, ident: &syn::Ident, attributes: &[syn::Attribute]) {
        let listed = self.names.iter().any(|name| ident == name);
        if listed
            && attributes
                .iter()
                .any(|attribute| mentions(attribute, &["no_mangle"]))
        {
            let line = ident.span().start().line;
            self.findings.push(
                Finding::at_line(
                    Class::ExportTwinOutsideFfi,
                    &self.source.path,
                    line,
                    self.source.line_text(line),
                )
                .about(&ident.to_string()),
            );
        }
    }
}

impl<'ast> Visit<'ast> for Twins<'_> {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.judge(&node.sig.ident, &node.attrs);
        visit::visit_item_fn(self, node);
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        self.judge(&node.sig.ident, &node.attrs);
        visit::visit_impl_item_fn(self, node);
    }

    fn visit_trait_item_fn(&mut self, node: &'ast syn::TraitItemFn) {
        self.judge(&node.sig.ident, &node.attrs);
        visit::visit_trait_item_fn(self, node);
    }

    fn visit_foreign_item_fn(&mut self, node: &'ast syn::ForeignItemFn) {
        self.judge(&node.sig.ident, &node.attrs);
        visit::visit_foreign_item_fn(self, node);
    }

    fn visit_item_static(&mut self, node: &'ast syn::ItemStatic) {
        self.judge(&node.ident, &node.attrs);
        visit::visit_item_static(self, node);
    }
}

/// `#[unsafe(no_mangle)]`.
fn is_no_mangle(function: &syn::ItemFn) -> bool {
    function.attrs.iter().any(|attribute| {
        matches!(attribute.style, syn::AttrStyle::Outer)
            && attribute.path().is_ident("unsafe")
            && attribute
                .parse_args::<syn::Ident>()
                .is_ok_and(|argument| argument == "no_mangle")
    })
}

/// How the sole call of a body names its callee.
enum Callee {
    /// `render_locked`, which resolves through the file's import.
    Bare,
    /// `crate::render_lock::render_locked`.
    FullPath,
    /// Another callee.
    Other,
}

/// The body's only statement, when it is a call.
fn sole_call(function: &syn::ItemFn) -> Option<&syn::ExprCall> {
    match function.block.stmts.as_slice() {
        [syn::Stmt::Expr(syn::Expr::Call(call), _)] => Some(call),
        _ => None,
    }
}

/// The callee of the body's only statement, when that statement is a call of one argument, a
/// closure that takes no parameter. `None` when the body is anything else.
fn callee_of_sole_call(function: &syn::ItemFn) -> Option<Callee> {
    let call = sole_call(function)?;
    let [syn::Expr::Closure(closure)] = call.args.iter().collect::<Vec<_>>().as_slice() else {
        return None;
    };
    if closure.asyncness.is_some() || !closure.inputs.is_empty() {
        return None;
    }
    let syn::Expr::Path(callee) = &*call.func else {
        return Some(Callee::Other);
    };
    if callee.qself.is_some() {
        return Some(Callee::Other);
    }
    let path = &callee.path;
    if path.is_ident(RENDER_LOCKED) {
        return Some(Callee::Bare);
    }
    let names: Vec<String> = path.segments.iter().map(|s| s.ident.to_string()).collect();
    let plain =
        path.leading_colon.is_none() && path.segments.iter().all(|s| s.arguments.is_empty());
    Some(
        if plain && names == ["crate", "render_lock", RENDER_LOCKED] {
            Callee::FullPath
        } else {
            Callee::Other
        },
    )
}

/// What the file does with the name `render_locked`.
struct Resolution {
    /// Whether a top-level `use` imports `crate::render_lock::render_locked` under its own name.
    imported: bool,
    /// The line of every other way the name is introduced, anywhere in the file.
    shadows: Vec<usize>,
    /// The line of each `#[cfg]` or `#[cfg_attr]` on a top-level `use` that imports the real
    /// function.
    conditional_imports: Vec<usize>,
}

impl Resolution {
    fn of(file: &syn::File) -> Self {
        let mut imported = false;
        let mut conditional_imports = Vec::new();
        for item in &file.items {
            if let syn::Item::Use(item) = item {
                let mut names = Vec::new();
                flatten(&item.tree, &mut Vec::new(), &mut names);
                if names.iter().any(|name| name.is_the_real_import()) {
                    imported = true;
                    conditional_imports.extend(crate::web_host::conditional(&item.attrs));
                }
            }
        }
        let mut finder = Shadows {
            lines: Vec::new(),
            members: false,
        };
        finder.visit_file(file);
        finder.lines.sort_unstable();
        finder.lines.dedup();
        Self {
            imported,
            shadows: finder.lines,
            conditional_imports,
        }
    }
}

/// One name that a `use` tree brings into scope.
struct Imported {
    /// The path the name is imported from, without a rename.
    path: Vec<String>,
    /// The name in scope.
    name: String,
    line: usize,
}

impl Imported {
    fn is_the_real_import(&self) -> bool {
        self.name == RENDER_LOCKED && self.path == ["crate", "render_lock", RENDER_LOCKED]
    }
}

fn flatten(tree: &syn::UseTree, prefix: &mut Vec<String>, names: &mut Vec<Imported>) {
    match tree {
        syn::UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            flatten(&path.tree, prefix, names);
            prefix.pop();
        }
        syn::UseTree::Name(name) => {
            let mut path = prefix.clone();
            path.push(name.ident.to_string());
            names.push(Imported {
                path,
                name: name.ident.to_string(),
                line: name.ident.span().start().line,
            });
        }
        syn::UseTree::Rename(rename) => {
            let mut path = prefix.clone();
            path.push(rename.ident.to_string());
            names.push(Imported {
                path,
                name: rename.rename.to_string(),
                line: rename.rename.span().start().line,
            });
        }
        syn::UseTree::Glob(_) => {}
        syn::UseTree::Group(group) => {
            for tree in &group.items {
                flatten(tree, prefix, names);
            }
        }
    }
}

/// The line of every introduction of the name `render_locked` in `file` other than the real
/// top-level import, `impl` and trait members included (#1528 D2), one entry per introduction.
pub(crate) fn introductions(file: &syn::File) -> Vec<usize> {
    let mut finder = Shadows {
        lines: Vec::new(),
        members: true,
    };
    finder.visit_file(file);
    finder.lines
}

/// Every introduction of the name `render_locked` other than the real top-level import.
struct Shadows {
    lines: Vec<usize>,
    /// Whether `impl` and trait members count too. A method does not shadow a bare call, so the
    /// wrap rule leaves them out; the uniqueness rule of `web_host` counts them.
    members: bool,
}

impl Shadows {
    fn named(&mut self, ident: &syn::Ident) {
        if ident == RENDER_LOCKED {
            self.lines.push(ident.span().start().line);
        }
    }
}

impl<'ast> Visit<'ast> for Shadows {
    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        let mut names = Vec::new();
        flatten(&node.tree, &mut Vec::new(), &mut names);
        for name in names {
            if name.name == RENDER_LOCKED && !name.is_the_real_import() {
                self.lines.push(name.line);
            }
        }
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.named(&node.sig.ident);
        visit::visit_item_fn(self, node);
    }

    fn visit_item_const(&mut self, node: &'ast syn::ItemConst) {
        self.named(&node.ident);
        visit::visit_item_const(self, node);
    }

    fn visit_item_static(&mut self, node: &'ast syn::ItemStatic) {
        self.named(&node.ident);
        visit::visit_item_static(self, node);
    }

    fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
        self.named(&node.ident);
        visit::visit_item_struct(self, node);
    }

    fn visit_item_enum(&mut self, node: &'ast syn::ItemEnum) {
        self.named(&node.ident);
        visit::visit_item_enum(self, node);
    }

    fn visit_item_union(&mut self, node: &'ast syn::ItemUnion) {
        self.named(&node.ident);
        visit::visit_item_union(self, node);
    }

    fn visit_item_type(&mut self, node: &'ast syn::ItemType) {
        self.named(&node.ident);
        visit::visit_item_type(self, node);
    }

    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        self.named(&node.ident);
        visit::visit_item_trait(self, node);
    }

    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        self.named(&node.ident);
        visit::visit_item_mod(self, node);
    }

    fn visit_item_macro(&mut self, node: &'ast syn::ItemMacro) {
        if let Some(ident) = &node.ident {
            self.named(ident);
        }
        visit::visit_item_macro(self, node);
    }

    fn visit_foreign_item_fn(&mut self, node: &'ast syn::ForeignItemFn) {
        self.named(&node.sig.ident);
        visit::visit_foreign_item_fn(self, node);
    }

    fn visit_pat_ident(&mut self, node: &'ast syn::PatIdent) {
        self.named(&node.ident);
        visit::visit_pat_ident(self, node);
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        if self.members {
            self.named(&node.sig.ident);
        }
        visit::visit_impl_item_fn(self, node);
    }

    fn visit_impl_item_const(&mut self, node: &'ast syn::ImplItemConst) {
        if self.members {
            self.named(&node.ident);
        }
        visit::visit_impl_item_const(self, node);
    }

    fn visit_impl_item_type(&mut self, node: &'ast syn::ImplItemType) {
        if self.members {
            self.named(&node.ident);
        }
        visit::visit_impl_item_type(self, node);
    }

    fn visit_trait_item_fn(&mut self, node: &'ast syn::TraitItemFn) {
        if self.members {
            self.named(&node.sig.ident);
        }
        visit::visit_trait_item_fn(self, node);
    }

    fn visit_trait_item_const(&mut self, node: &'ast syn::TraitItemConst) {
        if self.members {
            self.named(&node.ident);
        }
        visit::visit_trait_item_const(self, node);
    }

    fn visit_trait_item_type(&mut self, node: &'ast syn::TraitItemType) {
        if self.members {
            self.named(&node.ident);
        }
        visit::visit_trait_item_type(self, node);
    }
}
