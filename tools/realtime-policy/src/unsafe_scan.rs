//! The unsafe-ownership rule, read from tokens.

use proc_macro2::{Delimiter, TokenStream, TokenTree};

use crate::policy::Policy;
use crate::report::{Class, Finding};
use crate::source::Source;

/// Report every `unsafe` followed by `impl`, `fn`, `extern` or a brace group, unless the policy
/// allowlists the file. The scan reads macro token trees too. `unsafe trait` and the attribute
/// `#[unsafe(..)]` are outside the rule.
pub(crate) fn scan(source: &Source, policy: &Policy, findings: &mut Vec<Finding>) {
    if policy.unsafe_allowlist.contains(&source.path) {
        return;
    }
    scan_stream(&source.tokens, source, findings);
}

fn scan_stream(stream: &TokenStream, source: &Source, findings: &mut Vec<Finding>) {
    let mut tokens = stream.clone().into_iter().peekable();
    while let Some(token) = tokens.next() {
        match &token {
            TokenTree::Group(group) => scan_stream(&group.stream(), source, findings),
            TokenTree::Ident(ident) if ident == "unsafe" => {
                let opens_unsafe_code = match tokens.peek() {
                    Some(TokenTree::Ident(next)) => {
                        next == "impl" || next == "fn" || next == "extern"
                    }
                    Some(TokenTree::Group(group)) => group.delimiter() == Delimiter::Brace,
                    _ => false,
                };
                if opens_unsafe_code {
                    let line = ident.span().start().line;
                    findings.push(Finding::at_line(
                        Class::UnsafeOutsideOwnership,
                        &source.path,
                        line,
                        source.line_text(line),
                    ));
                }
            }
            _ => {}
        }
    }
}
