//! Identifier spelling.

/// The name an identifier stands for: a raw identifier (`r#try_pop`) is the identifier without
/// its prefix.
pub(super) fn bare(ident: &proc_macro2::Ident) -> String {
    let text = ident.to_string();
    match text.strip_prefix("r#") {
        Some(rest) => rest.to_owned(),
        None => text,
    }
}

/// Whether the token stream holds the identifier `name` at any depth.
pub(super) fn tokens_name(tokens: &proc_macro2::TokenStream, name: &str) -> bool {
    tokens.clone().into_iter().any(|token| match token {
        proc_macro2::TokenTree::Ident(ident) => bare(&ident) == name,
        proc_macro2::TokenTree::Group(group) => tokens_name(&group.stream(), name),
        _ => false,
    })
}
