//! Parsing a file into a syntax tree, and refusing what the tree cannot show.

use syn::spanned::Spanned;
use syn::visit::{self, Visit};

use crate::report::{Class, Finding};
use crate::source::Source;

/// Parse `source`. A file that does not parse, or whose tree holds a node that `syn` keeps as raw
/// tokens, is a finding: no rule can judge tokens the tree does not expose.
pub(crate) fn parse(source: &Source, findings: &mut Vec<Finding>) -> Option<syn::File> {
    let file = match syn::parse_file(&source.text) {
        Ok(file) => file,
        Err(error) => {
            findings.push(Finding::at_line_only(
                Class::ParseError,
                &source.path,
                error.span().start().line,
            ));
            return None;
        }
    };
    let mut finder = VerbatimFinder { source, findings };
    finder.visit_file(&file);
    Some(file)
}

/// Reports every node that the parser kept as a token stream.
struct VerbatimFinder<'a> {
    source: &'a Source,
    findings: &'a mut Vec<Finding>,
}

impl VerbatimFinder<'_> {
    fn report(&mut self, node: &impl Spanned) {
        let line = node.span().start().line;
        self.findings.push(Finding::at_line(
            Class::Verbatim,
            &self.source.path,
            line,
            self.source.line_text(line),
        ));
    }
}

impl<'ast> Visit<'ast> for VerbatimFinder<'_> {
    fn visit_item(&mut self, node: &'ast syn::Item) {
        if let syn::Item::Verbatim(tokens) = node {
            self.report(tokens);
        }
        visit::visit_item(self, node);
    }

    fn visit_trait_item(&mut self, node: &'ast syn::TraitItem) {
        if let syn::TraitItem::Verbatim(tokens) = node {
            self.report(tokens);
        }
        visit::visit_trait_item(self, node);
    }

    fn visit_impl_item(&mut self, node: &'ast syn::ImplItem) {
        if let syn::ImplItem::Verbatim(tokens) = node {
            self.report(tokens);
        }
        visit::visit_impl_item(self, node);
    }

    fn visit_foreign_item(&mut self, node: &'ast syn::ForeignItem) {
        if let syn::ForeignItem::Verbatim(tokens) = node {
            self.report(tokens);
        }
        visit::visit_foreign_item(self, node);
    }

    fn visit_expr(&mut self, node: &'ast syn::Expr) {
        // An empty one is `syn`'s spelling of a statement with no expression (`;;`): it holds
        // no tokens, so it hides nothing, and its span has no place to report.
        if let syn::Expr::Verbatim(tokens) = node
            && !tokens.is_empty()
        {
            self.report(tokens);
        }
        visit::visit_expr(self, node);
    }

    fn visit_pat(&mut self, node: &'ast syn::Pat) {
        if let syn::Pat::Verbatim(tokens) = node {
            self.report(tokens);
        }
        visit::visit_pat(self, node);
    }

    fn visit_type(&mut self, node: &'ast syn::Type) {
        if let syn::Type::Verbatim(tokens) = node {
            self.report(tokens);
        }
        visit::visit_type(self, node);
    }

    fn visit_type_param_bound(&mut self, node: &'ast syn::TypeParamBound) {
        if let syn::TypeParamBound::Verbatim(tokens) = node {
            self.report(tokens);
        }
        visit::visit_type_param_bound(self, node);
    }
}
