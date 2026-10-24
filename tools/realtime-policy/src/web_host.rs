//! What decides whether the `ffi.rs` that `render_wrap` reads is the code the browser module ships.
//!
//! The wrap rule reads `hosts/host-web/src/ffi.rs`. That proves nothing if the build can compile
//! something else: a `#[cfg]` that drops the wrapped export or the import and puts an unwrapped
//! twin in its place, an `#[export_name]` that gives a function a `miso_engine_web_v1_` name, a
//! `#[path]` that points `mod ffi;` at another file, or a glob `use` that supplies a pass-through
//! `render_locked` the rule never sees. Under `hosts/host-web/src` the tool refuses each written
//! form (#1496 D2 Amendment 1, rules (a) to (d)). Rule (b) refuses every `export_name` there,
//! whatever its value or spelling (#1528 D1(b)), so a symbol can only take a function's own
//! identifier; `render_wrap` refuses a generic listed export, which emits no symbol, and a
//! `no_mangle` twin of a listed name outside the top level of `ffi.rs` (D1(a), D1(c)). Together
//! they make the listed function the only written source of a listed symbol.
//!
//! The wrap is worth nothing if `render_locked` or the counting allocator is a pass-through in
//! the build that ships. [`scan_render_lock`] refuses conditional compilation on every top-level
//! item of `render_lock.rs` except three exact forms (the allocator, `#[cfg(test)] mod tests` and
//! the render-lock probe), every nested `cfg`, `cfg_attr` and `cfg!` there, an allocator of
//! another form, place or count, and a second `render_locked` (#1528 D2). The forms are compared
//! on the parsed attribute, never on its text. A name or attribute that a macro expansion
//! assembles from parts is #1438 limit 4.

use proc_macro2::{TokenStream, TokenTree};
use syn::punctuated::Punctuated;
use syn::visit::{self, Visit};

use crate::report::{Class, Finding};
use crate::source::Source;

/// The directory whose files the rules read, relative to the workspace root.
pub(crate) const WEB_SRC: &str = "hosts/host-web/src/";

/// The crate root, which declares the two modules.
pub(crate) const WEB_LIB: &str = "hosts/host-web/src/lib.rs";

/// The module that defines `render_locked`.
pub(crate) const WEB_RENDER_LOCK: &str = "hosts/host-web/src/render_lock.rs";

/// The two modules whose files the wrap rule reads: `(declared name, file)`.
const GUARDED_MODULES: [(&str, &str); 2] = [
    ("ffi", crate::render_wrap::WEB_FFI),
    ("render_lock", WEB_RENDER_LOCK),
];

/// A parsed file of `hosts/host-web/src`.
pub(crate) type Parsed<'a> = (&'a Source, &'a syn::File);

/// The line of the first `#[cfg]` or `#[cfg_attr]` among `attributes`: conditional compilation.
pub(crate) fn conditional(attributes: &[syn::Attribute]) -> Option<usize> {
    attributes
        .iter()
        .find(|attribute| attribute.path().is_ident("cfg") || attribute.path().is_ident("cfg_attr"))
        .map(|attribute| attribute.pound_token.spans[0].start().line)
}

/// Rule (a) for a guarded item: `#[cfg]` or `#[cfg_attr]` on `attributes` is a finding.
pub(crate) fn refuse_conditional(
    source: &Source,
    attributes: &[syn::Attribute],
    export: Option<&str>,
    findings: &mut Vec<Finding>,
) {
    if let Some(line) = conditional(attributes) {
        let finding = Finding::at_line(
            Class::ConditionalWrapPath,
            &source.path,
            line,
            source.line_text(line),
        );
        findings.push(match export {
            Some(export) => finding.about(export),
            None => finding,
        });
    }
}

/// Rules (a) and (c) on `lib.rs`, and (a) on the two files' inner attributes; (d) on both files.
pub(crate) fn scan_modules(files: &[Parsed<'_>], findings: &mut Vec<Finding>) {
    if let Some((source, file)) = find(files, WEB_LIB) {
        for (name, _) in GUARDED_MODULES {
            let declared = file.items.iter().find_map(|item| match item {
                syn::Item::Mod(item) if item.ident == name => Some(item),
                _ => None,
            });
            match declared {
                Some(item) if item.content.is_none() => {
                    refuse_conditional(source, &item.attrs, None, findings);
                    for attribute in &item.attrs {
                        if attribute.path().is_ident("path") {
                            let line = attribute.pound_token.spans[0].start().line;
                            findings.push(Finding::at_line(
                                Class::PathOnWebModule,
                                &source.path,
                                line,
                                source.line_text(line),
                            ));
                        }
                    }
                }
                _ => findings.push(Finding::at_path(Class::WebModuleNotDeclared, &source.path)),
            }
        }
    }
    for (_, path) in GUARDED_MODULES {
        let Some((source, file)) = find(files, path) else {
            continue;
        };
        // An inner `#![cfg(..)]` removes the module as `#[cfg]` on `mod ffi;` does.
        refuse_conditional(source, &file.attrs, None, findings);
        for item in &file.items {
            if let syn::Item::Use(item) = item
                && has_glob(&item.tree)
            {
                let line = item.use_token.span.start().line;
                findings.push(Finding::at_line(
                    Class::GlobInWrapModule,
                    &source.path,
                    line,
                    source.line_text(line),
                ));
            }
        }
    }
}

/// Rule (b): an `#[export_name]` with the table's prefix anywhere in `files`.
pub(crate) fn scan_export_names(files: &[Parsed<'_>], findings: &mut Vec<Finding>) {
    for (source, file) in files {
        let mut finder = ExportNames {
            source,
            findings: &mut *findings,
        };
        finder.visit_file(file);
    }
}

fn find<'a>(files: &[Parsed<'a>], path: &str) -> Option<Parsed<'a>> {
    files
        .iter()
        .find(|(source, _)| source.path == path)
        .copied()
}

/// Whether a `use` tree holds a glob, however deeply it is grouped.
fn has_glob(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Glob(_) => true,
        syn::UseTree::Path(path) => has_glob(&path.tree),
        syn::UseTree::Group(group) => group.items.iter().any(has_glob),
        syn::UseTree::Name(_) | syn::UseTree::Rename(_) => false,
    }
}

struct ExportNames<'a> {
    source: &'a Source,
    findings: &'a mut Vec<Finding>,
}

impl<'ast> Visit<'ast> for ExportNames<'_> {
    fn visit_attribute(&mut self, attribute: &'ast syn::Attribute) {
        if mentions(attribute, &["export_name"]) {
            let line = attribute.pound_token.spans[0].start().line;
            self.findings.push(Finding::at_line(
                Class::ExportNameOnWebExport,
                &self.source.path,
                line,
                self.source.line_text(line),
            ));
        }
        visit::visit_attribute(self, attribute);
    }
}

/// Whether `attribute`'s path, or any token of its argument list at any depth, is one of
/// `names`: the attribute itself, and the same inside `unsafe(..)`, `cfg_attr(.., ..)` or any
/// other list, whatever its value. A `name = value` attribute is judged by its path alone.
pub(crate) fn mentions(attribute: &syn::Attribute, names: &[&str]) -> bool {
    let path = attribute.path();
    if names.iter().any(|name| path.is_ident(name)) {
        return true;
    }
    match &attribute.meta {
        syn::Meta::List(list) => tokens_mention(&list.tokens, names),
        syn::Meta::Path(_) | syn::Meta::NameValue(_) => false,
    }
}

/// Whether any identifier of `tokens`, at any depth, is one of `names`.
fn tokens_mention(tokens: &TokenStream, names: &[&str]) -> bool {
    tokens.clone().into_iter().any(|tree| match tree {
        TokenTree::Ident(ident) => names.iter().any(|name| ident == name),
        TokenTree::Group(group) => tokens_mention(&group.stream(), names),
        TokenTree::Punct(_) | TokenTree::Literal(_) => false,
    })
}

/// The allocator's attributes apart from `doc`, in canonical form, sorted.
const ALLOCATOR_ATTRIBUTES: [&str; 2] = [
    "cfg(all(target_family=\"wasm\",not(test)))",
    "global_allocator",
];
/// The unit-test module's attributes apart from `doc`.
const TESTS_ATTRIBUTES: [&str; 1] = ["cfg(test)"];
/// The render-lock probe's attributes apart from `doc`, sorted (#1529 D3).
const PROBE_ATTRIBUTES: [&str; 2] = ["cfg(feature=\"render-lock-probe\")", "unsafe(no_mangle)"];
/// The render-lock probe's name (#1529 D3).
const PROBE: &str = "miso_engine_web_v1_render_lock_probe";
/// The function the wrap calls.
const RENDER_LOCKED: &str = "render_locked";

/// The canonical form of a parsed attribute argument: plain paths, lists of nested arguments and
/// `name = "string"` pairs, without spacing. `None` for any other shape, which matches no form.
fn canonical(meta: &syn::Meta) -> Option<String> {
    match meta {
        syn::Meta::Path(path) => plain_path(path),
        syn::Meta::List(list) => {
            let head = plain_path(&list.path)?;
            let arguments = list
                .parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
                .ok()?;
            let arguments: Option<Vec<String>> = arguments.iter().map(canonical).collect();
            Some(format!("{head}({})", arguments?.join(",")))
        }
        syn::Meta::NameValue(pair) => {
            let head = plain_path(&pair.path)?;
            let syn::Expr::Lit(syn::ExprLit {
                attrs,
                lit: syn::Lit::Str(value),
            }) = &pair.value
            else {
                return None;
            };
            attrs
                .is_empty()
                .then(|| format!("{head}={:?}", value.value()))
        }
    }
}

fn plain_path(path: &syn::Path) -> Option<String> {
    if path.leading_colon.is_some() || path.segments.iter().any(|s| !s.arguments.is_empty()) {
        return None;
    }
    Some(
        path.segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .join("::"),
    )
}

/// Whether the attributes of an item, apart from `doc`, are outer and exactly `expected` (sorted).
fn exactly(attributes: &[syn::Attribute], expected: &[&str]) -> bool {
    let mut found = Vec::new();
    for attribute in attributes {
        if attribute.path().is_ident("doc") {
            continue;
        }
        let Some(form) = canonical(&attribute.meta) else {
            return false;
        };
        if !matches!(attribute.style, syn::AttrStyle::Outer) {
            return false;
        }
        found.push(form);
    }
    found.sort();
    found == expected
}

/// Whether `module` carries exactly `#[cfg(test)]` among its attributes.
fn is_cfg_test(attributes: &[syn::Attribute]) -> bool {
    attributes
        .iter()
        .any(|attribute| canonical(&attribute.meta).is_some_and(|form| form == TESTS_ATTRIBUTES[0]))
}

/// The outer attributes of a top-level item.
fn item_attributes(item: &syn::Item) -> &[syn::Attribute] {
    match item {
        syn::Item::Const(item) => &item.attrs,
        syn::Item::Enum(item) => &item.attrs,
        syn::Item::ExternCrate(item) => &item.attrs,
        syn::Item::Fn(item) => &item.attrs,
        syn::Item::ForeignMod(item) => &item.attrs,
        syn::Item::Impl(item) => &item.attrs,
        syn::Item::Macro(item) => &item.attrs,
        syn::Item::Mod(item) => &item.attrs,
        syn::Item::Static(item) => &item.attrs,
        syn::Item::Struct(item) => &item.attrs,
        syn::Item::Trait(item) => &item.attrs,
        syn::Item::TraitAlias(item) => &item.attrs,
        syn::Item::Type(item) => &item.attrs,
        syn::Item::Union(item) => &item.attrs,
        syn::Item::Use(item) => &item.attrs,
        // `syntax::parse` refuses a verbatim node; it carries no attribute the rule could read.
        _ => &[],
    }
}

/// What a top-level item of `render_lock.rs` is to the D2 rule.
enum Form {
    /// An item with a `global_allocator` attribute; `true` when it has the exact form.
    Allocator(bool),
    /// `#[cfg(test)] mod tests { .. }`, inline, exactly.
    Tests,
    /// The render-lock probe, exactly.
    Probe,
    /// Anything else.
    Other,
}

impl Form {
    fn of(item: &syn::Item) -> Self {
        let attributes = item_attributes(item);
        if attributes
            .iter()
            .any(|attribute| mentions(attribute, &["global_allocator"]))
        {
            let exact = matches!(item, syn::Item::Static(item)
                if matches!(item.mutability, syn::StaticMutability::None)
                    && exactly(attributes, &ALLOCATOR_ATTRIBUTES)
                    && is_allocator_type(&item.ty));
            return Self::Allocator(exact);
        }
        match item {
            syn::Item::Mod(module)
                if module.ident == "tests"
                    && module.content.is_some()
                    && exactly(attributes, &TESTS_ATTRIBUTES) =>
            {
                Self::Tests
            }
            syn::Item::Fn(function)
                if function.sig.ident == PROBE
                    && matches!(function.vis, syn::Visibility::Public(_))
                    && function.sig.abi.as_ref().is_some_and(|abi| {
                        abi.name.as_ref().is_some_and(|name| name.value() == "C")
                    })
                    && exactly(attributes, &PROBE_ATTRIBUTES) =>
            {
                Self::Probe
            }
            _ => Self::Other,
        }
    }
}

/// `RenderLockedAllocator<std::alloc::System>`, written as plain paths.
fn is_allocator_type(ty: &syn::Type) -> bool {
    let syn::Type::Path(outer) = ty else {
        return false;
    };
    let path = &outer.path;
    if outer.qself.is_some() || path.leading_colon.is_some() {
        return false;
    }
    let [segment] = path.segments.iter().collect::<Vec<_>>()[..] else {
        return false;
    };
    let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return false;
    };
    let [syn::GenericArgument::Type(syn::Type::Path(inner))] =
        arguments.args.iter().collect::<Vec<_>>()[..]
    else {
        return false;
    };
    segment.ident == "RenderLockedAllocator"
        && inner.qself.is_none()
        && plain_path(&inner.path).is_some_and(|inner| inner == "std::alloc::System")
}

/// D2: the conditional-compilation, allocator and `render_locked` rules of `render_lock.rs`, and
/// the allocator and `render_locked` rules on the other files of `files`.
pub(crate) fn scan_render_lock(files: &[Parsed<'_>], findings: &mut Vec<Finding>) {
    let mut skipped: Vec<*const syn::Item> = Vec::new();
    if let Some((source, file)) = find(files, WEB_RENDER_LOCK) {
        let at = |class, line: usize| {
            Finding::at_line(class, &source.path, line, source.line_text(line))
        };
        let mut allocators = 0;
        let mut probe = false;
        let mut tests = None;
        for item in &file.items {
            let attributes = item_attributes(item);
            let line = first_line(attributes, item);
            match Form::of(item) {
                Form::Allocator(exact) => {
                    allocators += 1;
                    skipped.push(item);
                    if !exact {
                        findings.push(at(Class::RenderLockAllocatorForm, line));
                    }
                }
                Form::Tests if tests.is_none() => tests = Some(item),
                Form::Probe if !probe => probe = true,
                Form::Tests | Form::Probe | Form::Other => {
                    if let Some(line) = conditional(attributes) {
                        findings.push(at(Class::ConditionalRenderLockItem, line));
                    }
                }
            }
        }
        if allocators != 1 {
            findings.push(Finding::at_path(
                Class::RenderLockAllocatorForm,
                &source.path,
            ));
        }
        let mut nested = Nested {
            source,
            own: Vec::new(),
            findings: &mut *findings,
        };
        for item in &file.items {
            if tests.is_some_and(|tests| std::ptr::eq(tests, item)) {
                continue;
            }
            nested.own = item_attributes(item)
                .iter()
                .map(std::ptr::from_ref)
                .collect();
            nested.visit_item(item);
        }
        let defined: Vec<usize> = file
            .items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Fn(function) if function.sig.ident == RENDER_LOCKED => {
                    Some(function.sig.ident.span().start().line)
                }
                _ => None,
            })
            .collect();
        let mut others = crate::render_wrap::introductions(file);
        if let [line] = defined.as_slice() {
            if let Some(index) = others.iter().position(|other| other == line) {
                others.remove(index);
            }
        } else {
            findings.push(Finding::at_path(Class::RenderLockedNotUnique, &source.path));
        }
        for line in others {
            findings.push(at(Class::RenderLockedNotUnique, line));
        }
    }
    for (source, file) in files {
        if source.path != crate::render_wrap::WEB_FFI && source.path != WEB_RENDER_LOCK {
            for line in crate::render_wrap::introductions(file) {
                findings.push(Finding::at_line(
                    Class::RenderLockedNotUnique,
                    &source.path,
                    line,
                    source.line_text(line),
                ));
            }
        }
        let mut allocators = Allocators {
            source,
            skipped: if source.path == WEB_RENDER_LOCK {
                &skipped
            } else {
                &[]
            },
            in_test: 0,
            findings: &mut *findings,
        };
        allocators.visit_file(file);
    }
}

/// The line of the item's first attribute, or of the item when it has none.
fn first_line(attributes: &[syn::Attribute], item: &syn::Item) -> usize {
    attributes.first().map_or_else(
        || syn::spanned::Spanned::span(item).start().line,
        |attribute| attribute.pound_token.spans[0].start().line,
    )
}

/// D2, nested sites of `render_lock.rs`: every `cfg` or `cfg_attr` attribute other than the
/// top-level item's own, and every macro invocation whose path or tokens name `cfg` or
/// `cfg_attr` (`cfg!`, or a `#[cfg]` inside `thread_local!`).
struct Nested<'a> {
    source: &'a Source,
    /// The attributes of the top-level item being visited, which the top-level rule judges.
    own: Vec<*const syn::Attribute>,
    findings: &'a mut Vec<Finding>,
}

const CONDITIONAL: [&str; 2] = ["cfg", "cfg_attr"];

impl Nested<'_> {
    fn push(&mut self, line: usize) {
        self.findings.push(Finding::at_line(
            Class::ConditionalInRenderLockBody,
            &self.source.path,
            line,
            self.source.line_text(line),
        ));
    }
}

impl<'ast> Visit<'ast> for Nested<'_> {
    fn visit_attribute(&mut self, attribute: &'ast syn::Attribute) {
        if !self.own.contains(&std::ptr::from_ref(attribute)) && mentions(attribute, &CONDITIONAL) {
            self.push(attribute.pound_token.spans[0].start().line);
        }
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        let named = node
            .path
            .segments
            .last()
            .is_some_and(|segment| CONDITIONAL.iter().any(|name| segment.ident == name));
        if named || tokens_mention(&node.tokens, &CONDITIONAL) {
            self.push(syn::spanned::Spanned::span(&node.path).start().line);
        }
    }
}

/// D2: a `global_allocator` attribute outside the top-level allocator of `render_lock.rs` and
/// outside a `#[cfg(test)]` module.
struct Allocators<'a> {
    source: &'a Source,
    /// The top-level allocator items of `render_lock.rs`, which the form rule judges.
    skipped: &'a [*const syn::Item],
    in_test: usize,
    findings: &'a mut Vec<Finding>,
}

impl<'ast> Visit<'ast> for Allocators<'_> {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if !self.skipped.contains(&std::ptr::from_ref(item)) {
            visit::visit_item(self, item);
        }
    }

    fn visit_item_mod(&mut self, module: &'ast syn::ItemMod) {
        let test = is_cfg_test(&module.attrs);
        self.in_test += usize::from(test);
        visit::visit_item_mod(self, module);
        self.in_test -= usize::from(test);
    }

    fn visit_attribute(&mut self, attribute: &'ast syn::Attribute) {
        if self.in_test == 0 && mentions(attribute, &["global_allocator"]) {
            let line = attribute.pound_token.spans[0].start().line;
            self.findings.push(Finding::at_line(
                Class::AllocatorOutsideRenderLock,
                &self.source.path,
                line,
                self.source.line_text(line),
            ));
        }
    }
}
