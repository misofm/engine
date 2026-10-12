//! What decides whether the `ffi.rs` that `render_wrap` reads is the code the browser module ships.
//!
//! The wrap rule reads `hosts/host-web/src/ffi.rs`. That proves nothing if the build can compile
//! something else: a `#[cfg]` that drops the wrapped export or the import and puts an unwrapped
//! twin in its place, an `#[export_name]` that gives a function a `miso_engine_web_v1_` name, a
//! `#[path]` that points `mod ffi;` at another file, or a glob `use` that supplies a pass-through
//! `render_locked` the rule never sees. Under `hosts/host-web/src` the tool refuses each written
//! form (#1496 D2 Amendment 1, rules (a) to (d)). Rule (b) refuses only an `export_name` whose
//! value starts with the prefix; that every exported name comes from a listed function's own
//! identifier is not enforced (see the successor of #1496). A name or attribute that a macro
//! expansion assembles from parts is #1438 limit 4.

use syn::punctuated::Punctuated;
use syn::visit::{self, Visit};

use crate::report::{Class, Finding};
use crate::source::Source;
use crate::table::PREFIX;

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
        let mut names = Vec::new();
        export_names(&attribute.meta, &mut names);
        if names.iter().any(|name| name.starts_with(PREFIX)) {
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

/// The string values of every `export_name` that `meta` sets, through `unsafe(..)` and
/// `cfg_attr(.., ..)`.
fn export_names(meta: &syn::Meta, names: &mut Vec<String>) {
    match meta {
        syn::Meta::NameValue(pair) if pair.path.is_ident("export_name") => {
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(value),
                ..
            }) = &pair.value
            {
                names.push(value.value());
            }
        }
        syn::Meta::List(list) if list.path.is_ident("unsafe") => {
            for inner in nested(list).iter() {
                export_names(inner, names);
            }
        }
        syn::Meta::List(list) if list.path.is_ident("cfg_attr") => {
            // The first argument is the condition.
            for inner in nested(list).iter().skip(1) {
                export_names(inner, names);
            }
        }
        _ => {}
    }
}

fn nested(list: &syn::MetaList) -> Punctuated<syn::Meta, syn::Token![,]> {
    list.parse_args_with(Punctuated::parse_terminated)
        .unwrap_or_default()
}
