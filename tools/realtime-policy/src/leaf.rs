//! A file's token stream flattened to leaves: the shape that every token rule reads.
//!
//! A leaf is an identifier or keyword, a literal, one punctuation character, or one delimiter of a
//! group. A group itself is not a leaf, because its span covers every line inside it.

use proc_macro2::{Delimiter, LineColumn, Literal, TokenStream, TokenTree};

/// What a leaf is.
#[derive(Clone, Debug)]
pub(crate) enum Kind {
    /// An identifier or a keyword (the lexer does not tell them apart).
    Word(String),
    /// A literal of any kind.
    Literal(Literal),
    /// One punctuation character.
    Punct(char),
    /// The opening delimiter of a group.
    Open(char),
    /// The closing delimiter of a group.
    Close(char),
}

/// One leaf token with its extent. `end` is one past its last character.
#[derive(Clone, Debug)]
pub(crate) struct Leaf {
    pub(crate) start: LineColumn,
    pub(crate) end: LineColumn,
    pub(crate) kind: Kind,
}

/// The leaves of `tokens` in source order: a group gives its opening delimiter, its contents and
/// its closing delimiter.
pub(crate) fn flatten(tokens: &TokenStream) -> Vec<Leaf> {
    let mut leaves = Vec::new();
    push_all(tokens, &mut leaves);
    leaves
}

fn push_all(tokens: &TokenStream, leaves: &mut Vec<Leaf>) {
    for token in tokens.clone() {
        match token {
            TokenTree::Group(group) => {
                let (open, close) = delimiters(group.delimiter());
                if let Some(open) = open {
                    let span = group.span_open();
                    leaves.push(Leaf {
                        start: span.start(),
                        end: span.end(),
                        kind: Kind::Open(open),
                    });
                }
                push_all(&group.stream(), leaves);
                if let Some(close) = close {
                    let span = group.span_close();
                    leaves.push(Leaf {
                        start: span.start(),
                        end: span.end(),
                        kind: Kind::Close(close),
                    });
                }
            }
            TokenTree::Ident(ident) => {
                let span = ident.span();
                leaves.push(Leaf {
                    start: span.start(),
                    end: span.end(),
                    kind: Kind::Word(ident.to_string()),
                });
            }
            TokenTree::Punct(punct) => {
                let span = punct.span();
                leaves.push(Leaf {
                    start: span.start(),
                    end: span.end(),
                    kind: Kind::Punct(punct.as_char()),
                });
            }
            TokenTree::Literal(literal) => {
                let span = literal.span();
                leaves.push(Leaf {
                    start: span.start(),
                    end: span.end(),
                    kind: Kind::Literal(literal),
                });
            }
        }
    }
}

/// The delimiter characters of a group; a group with no delimiter has none.
fn delimiters(delimiter: Delimiter) -> (Option<char>, Option<char>) {
    match delimiter {
        Delimiter::Parenthesis => (Some('('), Some(')')),
        Delimiter::Brace => (Some('{'), Some('}')),
        Delimiter::Bracket => (Some('['), Some(']')),
        Delimiter::None => (None, None),
    }
}
