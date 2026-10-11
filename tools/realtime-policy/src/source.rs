//! One `.rs` file, read and lexed.

use std::fs;
use std::path::Path;
use std::str::FromStr;

use proc_macro2::TokenStream;

use crate::report::{Class, Finding};

/// A file that is valid UTF-8 and lexes.
pub(crate) struct Source {
    /// The path relative to the workspace root, with `/` separators.
    pub(crate) path: String,
    /// The file's text.
    pub(crate) text: String,
    /// The text's token stream.
    pub(crate) tokens: TokenStream,
    /// The byte offset of each line's first byte; line `n` (1-based) starts at `line_starts[n - 1]`.
    line_starts: Vec<usize>,
}

impl Source {
    /// Read and lex `absolute`; `path` is how findings name it.
    pub(crate) fn load(absolute: &Path, path: &str) -> Result<Self, Finding> {
        let bytes =
            fs::read(absolute).map_err(|_| Finding::at_path(Class::UnreadableFile, path))?;
        let text = String::from_utf8(bytes).map_err(|_| Finding::at_path(Class::NotUtf8, path))?;
        let tokens = TokenStream::from_str(&text).map_err(|error| {
            Finding::at_line_only(Class::LexError, path, error.span().start().line)
        })?;
        let mut line_starts = vec![0];
        line_starts.extend(text.match_indices('\n').map(|(offset, _)| offset + 1));
        Ok(Self {
            path: path.to_owned(),
            text,
            tokens,
            line_starts,
        })
    }

    /// The number of lines, counting by `\n` as the lexer does.
    pub(crate) fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    /// The text of line `line` (1-based) without its `\n` and any `\r` before it.
    pub(crate) fn line_text(&self, line: usize) -> &str {
        let start = self.line_starts[line - 1];
        let end = self
            .line_starts
            .get(line)
            .map_or(self.text.len(), |next| next - 1);
        self.text[start..end]
            .strip_suffix('\r')
            .unwrap_or(&self.text[start..end])
    }

    /// The byte offset of `column` (counted in characters) on `line` (1-based).
    pub(crate) fn offset(&self, line: usize, column: usize) -> usize {
        let start = self.line_starts[line - 1];
        let rest = &self.text[start..];
        rest.char_indices()
            .nth(column)
            .map_or(self.text.len(), |(index, _)| start + index)
    }
}
