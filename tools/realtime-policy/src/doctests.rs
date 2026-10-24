//! Rule (iv) of #1527 (#1831): compiled doctest code blocks follow the unsafe-ownership rule of
//! the file that holds them, and hold no `include!`, `include_str!` or `include_bytes!`.
//!
//! The lexer gives every doc form (`///`, `//!`, `/** */`, `/*! */` and a written
//! `#[doc = ".."]`) as a `doc` attribute whose text is a string literal, which no other rule
//! reads. This module joins the doc attributes of one item, finds the code blocks that rustdoc
//! compiles (a fenced block, also behind `>` or a list marker, or an indented block, whose
//! language string makes it Rust and not `ignore`), strips the hidden-line `# ` prefix, and
//! re-lexes each block with the same lexer. A block that does not lex is a finding (fail closed).
//!
//! The scan reads only doc text that is written in the source as a string literal, inside macro
//! token trees too. A doc text that a macro builds (`#[doc = concat!(..)]`, a `doc = $x`
//! metavariable, `cfg_attr(.., doc = ..)`, `#[doc = include_str!(..)]`) is a stated limit, as
//! for the tool's other rules: the tool refuses the direct written forms. `#[cfg]` is not honoured:
//! a block on an item that is disabled on this host is compiled on another target. The markdown
//! reading is a CommonMark subset, which `blocks` states.

use std::str::FromStr;

use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};

use crate::policy::Policy;
use crate::report::{Class, Finding};
use crate::source::Source;
use crate::unsafe_scan;

/// One line of an item's doc text and the source line it comes from.
struct DocLine {
    text: String,
    line: usize,
}

/// A compiled code block: its code lines (hidden-line prefix stripped) with their source lines,
/// and the source line of the block's first line (the fence, or the first indented line).
struct Block {
    code: Vec<(String, usize)>,
    line: usize,
}

/// Check every compiled doctest block of `source`.
pub(crate) fn scan(source: &Source, policy: &Policy, findings: &mut Vec<Finding>) {
    let unsafe_allowed = policy.unsafe_allowlist.contains(&source.path);
    let mut docs = Vec::new();
    collect(&source.tokens, source, &mut docs);
    for doc in &docs {
        for block in blocks(doc) {
            check_block(&block, source, unsafe_allowed, findings);
        }
    }
}

fn check_block(block: &Block, source: &Source, unsafe_allowed: bool, findings: &mut Vec<Finding>) {
    let code: Vec<&str> = block.code.iter().map(|(text, _)| text.as_str()).collect();
    let Ok(tokens) = TokenStream::from_str(&code.join("\n")) else {
        findings.push(Finding::at_line(
            Class::DoctestNotLexable,
            &source.path,
            block.line,
            source.line_text(block.line),
        ));
        return;
    };
    // A span line is 1-based in the block's code; past the last line (an empty block) it maps to
    // the block's own line.
    let source_line = |line: usize| {
        line.checked_sub(1)
            .and_then(|index| block.code.get(index))
            .map_or(block.line, |(_, line)| *line)
    };
    let mut push = |class, line: usize| {
        let line = source_line(line);
        findings.push(Finding::at_line(
            class,
            &source.path,
            line,
            source.line_text(line),
        ));
    };
    if !unsafe_allowed {
        unsafe_scan::scan_stream(&tokens, &mut |line| push(Class::UnsafeInDoctest, line));
    }
    scan_includes(&tokens, &mut |line| push(Class::IncludeInDoctest, line));
}

/// Report every identifier `include`, `include_str` or `include_bytes`, raw or not, in token trees
/// too.
fn scan_includes(stream: &TokenStream, found: &mut dyn FnMut(usize)) {
    for token in stream.clone() {
        match &token {
            TokenTree::Group(group) => scan_includes(&group.stream(), found),
            TokenTree::Ident(ident) => {
                let name = ident.to_string();
                let name = name.strip_prefix("r#").unwrap_or(&name);
                if matches!(name, "include" | "include_str" | "include_bytes") {
                    found(ident.span().start().line);
                }
            }
            _ => {}
        }
    }
}

/// Collect the doc text of every item in `stream`, groups included: a run of attributes of one
/// kind (outer or inner) is one item's doc, and a token that is not an attribute ends the run.
fn collect(stream: &TokenStream, source: &Source, docs: &mut Vec<Vec<DocLine>>) {
    let tokens: Vec<TokenTree> = stream.clone().into_iter().collect();
    let mut run: Vec<Vec<DocLine>> = Vec::new();
    let mut run_inner = None;
    let mut index = 0;
    while index < tokens.len() {
        if let Some((inner, group, next)) = attribute_at(&tokens, index) {
            if run_inner.is_some_and(|kind| kind != inner) {
                flush(&mut run, docs);
            }
            run_inner = Some(inner);
            match doc_fragment(group, source) {
                Some(fragment) => run.push(fragment),
                None => collect(&group.stream(), source, docs),
            }
            index = next;
            continue;
        }
        flush(&mut run, docs);
        run_inner = None;
        if let TokenTree::Group(group) = &tokens[index] {
            collect(&group.stream(), source, docs);
        }
        index += 1;
    }
    flush(&mut run, docs);
}

/// Join a run's fragments into one doc text, unindented as rustdoc does: the smallest indent of
/// any non-blank line is removed from every line.
fn flush(run: &mut Vec<Vec<DocLine>>, docs: &mut Vec<Vec<DocLine>>) {
    if run.is_empty() {
        return;
    }
    let mut lines: Vec<DocLine> = run.drain(..).flatten().collect();
    let indent = lines
        .iter()
        .filter(|line| !line.text.trim().is_empty())
        .map(|line| indent_width(&line.text))
        .min()
        .unwrap_or(0);
    for line in &mut lines {
        line.text = strip_width(&line.text, indent).to_owned();
    }
    docs.push(lines);
}

/// An attribute `#[..]` or `#![..]` at `index`: whether it is inner, its bracket group, and the
/// index after it.
fn attribute_at(tokens: &[TokenTree], index: usize) -> Option<(bool, &Group, usize)> {
    let TokenTree::Punct(hash) = tokens.get(index)? else {
        return None;
    };
    if hash.as_char() != '#' {
        return None;
    }
    let (inner, at) = match tokens.get(index + 1)? {
        TokenTree::Punct(bang) if bang.as_char() == '!' => (true, index + 2),
        _ => (false, index + 1),
    };
    match tokens.get(at)? {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
            Some((inner, group, at + 1))
        }
        _ => None,
    }
}

/// The lines of a `doc = "<string literal>"` attribute body, with their source lines. A block
/// comment (`/** */`, `/*! */`) is beautified as rustc does: a blank first and last line goes, and
/// a leading `*` column is stripped.
fn doc_fragment(group: &Group, source: &Source) -> Option<Vec<DocLine>> {
    let tokens: Vec<TokenTree> = group.stream().into_iter().collect();
    let [
        TokenTree::Ident(doc),
        TokenTree::Punct(equals),
        TokenTree::Literal(literal),
    ] = tokens.as_slice()
    else {
        return None;
    };
    if doc != "doc" || equals.as_char() != '=' {
        return None;
    }
    let value = syn::parse2::<syn::LitStr>(TokenStream::from(TokenTree::Literal(literal.clone())))
        .ok()?
        .value();
    let start = literal.span().start();
    let end = literal.span().end().line;
    let mut lines: Vec<DocLine> = value
        .split('\n')
        .enumerate()
        .map(|(offset, text)| DocLine {
            text: text.strip_suffix('\r').unwrap_or(text).to_owned(),
            line: (start.line + offset).min(end),
        })
        .collect();
    if source.text[source.offset(start.line, start.column)..].starts_with("/*") {
        beautify(&mut lines);
    }
    Some(lines)
}

fn beautify(lines: &mut Vec<DocLine>) {
    let only_stars = |line: &DocLine| line.text.trim().trim_matches('*').trim().is_empty();
    if lines.len() > 1 && lines.first().is_some_and(only_stars) {
        lines.remove(0);
    }
    if lines.len() > 1 && lines.last().is_some_and(only_stars) {
        lines.pop();
    }
    let starred = lines
        .iter()
        .filter(|line| !line.text.trim().is_empty())
        .all(|line| line.text.trim_start().starts_with('*'));
    if starred {
        for line in lines.iter_mut() {
            if let Some(rest) = line.text.trim_start().strip_prefix('*') {
                line.text = rest.to_owned();
            }
        }
    }
}

/// The width of a line's leading spaces and tabs, a tab counting 4.
fn indent_width(text: &str) -> usize {
    text.chars()
        .take_while(|c| *c == ' ' || *c == '\t')
        .map(|c| if c == '\t' { 4 } else { 1 })
        .sum()
}

/// `text` with up to `width` of its leading indent removed.
fn strip_width(text: &str, width: usize) -> &str {
    let mut removed = 0;
    for (index, c) in text.char_indices() {
        if removed >= width || (c != ' ' && c != '\t') {
            return &text[index..];
        }
        removed += if c == '\t' { 4 } else { 1 };
    }
    ""
}

/// The container a fenced block opened in, which each of its lines must continue.
#[derive(Clone, Copy)]
enum Container {
    /// No container: the block's lines are read as they are.
    None,
    /// A block quote of this depth: each line must open with as many `>`.
    Quote(usize),
    /// A list item whose content starts at this width: each non-blank line must be indented by it.
    List(usize),
}

/// An open fenced block.
struct Fence {
    marker: char,
    length: usize,
    container: Container,
    /// Whether the fence was opened inside a list item: its closing fence may be indented more.
    in_list: bool,
    compiled: bool,
    block: Block,
}

/// The compiled code blocks of one item's doc text. The reading is a CommonMark subset: a fence
/// of 3 or more backticks or tildes (up to 3 spaces of indent, any indent inside a list item),
/// also behind `>` (nested) or a list marker (`-`, `*`, `+`, `1.`, `1)`), closed by a fence of the
/// same character at least as long, or by the end of the text or of its container; and an indented
/// block (4 spaces, past the list content indent inside a list item, also behind `>`) that follows
/// a blank line. An indented block inside a block quote inside a list item is not recognized.
fn blocks(doc: &[DocLine]) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut fence: Option<Fence> = None;
    // An open indented block, the block-quote depth it sits at, and its indent threshold.
    let mut indented: Option<(Block, usize, usize)> = None;
    let mut previous_blank = true;
    let mut previous_depth = 0;
    // The content indent of the list item the text is in, if any.
    let mut list: Option<usize> = None;
    for line in doc {
        let text = line.text.as_str();
        if let Some(open) = &mut fence {
            match continue_container(text, open.container) {
                Some(rest) if is_closing(rest, open.marker, open.length, open.in_list) => {
                    finish(fence.take(), &mut blocks);
                    previous_blank = false;
                    continue;
                }
                Some(rest) => {
                    open.block.code.push((hidden_stripped(rest), line.line));
                    continue;
                }
                None => finish(fence.take(), &mut blocks),
            }
        }
        if let Some((block, depth, threshold)) = &mut indented {
            let container = if *depth == 0 {
                Container::None
            } else {
                Container::Quote(*depth)
            };
            match continue_container(text, container) {
                Some(rest) if rest.trim().is_empty() || indent_width(rest) >= *threshold => {
                    let code = hidden_stripped(strip_width(rest, *threshold));
                    block.code.push((code, line.line));
                    continue;
                }
                _ => {
                    let (block, _, _) = indented.take().expect("an open indented block");
                    blocks.push(block);
                }
            }
        }
        let (container, rest) = open_container(text);
        let depth = match container {
            Container::Quote(depth) => depth,
            _ => 0,
        };
        let starts_block = previous_blank || depth != previous_depth;
        previous_depth = depth;
        if rest.trim().is_empty() {
            previous_blank = true;
            continue;
        }
        let in_list = depth == 0 && (list.is_some() || matches!(container, Container::List(_)));
        if let Some((marker, length, info)) = opening(rest, in_list) {
            fence = Some(Fence {
                marker,
                length,
                container,
                in_list,
                compiled: compiled(info),
                block: Block {
                    code: Vec::new(),
                    line: line.line,
                },
            });
            if let Container::List(width) = container {
                list = Some(width);
            }
            previous_blank = false;
            continue;
        }
        let threshold = match (depth, list) {
            (0, Some(width)) => width + 4,
            _ => 4,
        };
        if starts_block
            && !matches!(container, Container::List(_))
            && indent_width(rest) >= threshold
        {
            let code = hidden_stripped(strip_width(rest, threshold));
            let block = Block {
                code: vec![(code, line.line)],
                line: line.line,
            };
            indented = Some((block, depth, threshold));
            continue;
        }
        match container {
            Container::List(width) => list = Some(width),
            Container::None if indent_width(text) < list.unwrap_or(0) => list = None,
            _ => {}
        }
        previous_blank = false;
    }
    finish(fence, &mut blocks);
    if let Some((block, _, _)) = indented {
        blocks.push(block);
    }
    blocks
}

fn finish(fence: Option<Fence>, blocks: &mut Vec<Block>) {
    if let Some(fence) = fence
        && fence.compiled
    {
        blocks.push(fence.block);
    }
}

/// The container a line opens, and the rest of the line after its markers.
fn open_container(text: &str) -> (Container, &str) {
    let mut depth = 0;
    let mut rest = text;
    while let Some(after) = quote_marker(rest) {
        depth += 1;
        rest = after;
    }
    if depth > 0 {
        return (Container::Quote(depth), rest);
    }
    match list_marker(text) {
        Some(width) => (Container::List(width), &text[width..]),
        None => (Container::None, text),
    }
}

/// The rest of a line after a `>` marker (up to 3 spaces before it, one optional space after).
fn quote_marker(text: &str) -> Option<&str> {
    let trimmed = text.trim_start_matches(' ');
    if text.len() - trimmed.len() > 3 {
        return None;
    }
    let after = trimmed.strip_prefix('>')?;
    Some(after.strip_prefix(' ').unwrap_or(after))
}

/// The byte width of a list marker and the spaces after it (one to four), if `text` opens a list
/// item.
fn list_marker(text: &str) -> Option<usize> {
    let indent = text.len() - text.trim_start_matches(' ').len();
    if indent > 3 {
        return None;
    }
    let body = &text[indent..];
    let marker = if body.starts_with(['-', '*', '+']) {
        1
    } else {
        let digits = body.chars().take_while(char::is_ascii_digit).count();
        if !(1..=9).contains(&digits) || !body[digits..].starts_with(['.', ')']) {
            return None;
        }
        digits + 1
    };
    let after = &body[marker..];
    let spaces = after.len() - after.trim_start_matches(' ').len();
    if !(1..=4).contains(&spaces) {
        return None;
    }
    Some(indent + marker + spaces)
}

/// The rest of a line inside an open fence's container, or `None` when the line ends it.
fn continue_container(text: &str, container: Container) -> Option<&str> {
    match container {
        Container::None => Some(text),
        Container::Quote(depth) => {
            let mut rest = text;
            for _ in 0..depth {
                rest = quote_marker(rest)?;
            }
            Some(rest)
        }
        Container::List(width) => {
            if text.trim().is_empty() {
                Some("")
            } else if indent_width(text) >= width {
                Some(strip_width(text, width))
            } else {
                None
            }
        }
    }
}

/// An opening fence: its character, its length and its info string.
fn opening(text: &str, in_list: bool) -> Option<(char, usize, &str)> {
    let trimmed = text.trim_start_matches(' ');
    if !in_list && text.len() - trimmed.len() > 3 {
        return None;
    }
    let trimmed = if in_list { text.trim_start() } else { trimmed };
    let marker = trimmed.chars().next().filter(|c| *c == '`' || *c == '~')?;
    let length = trimmed.chars().take_while(|c| *c == marker).count();
    if length < 3 {
        return None;
    }
    let info = &trimmed[length..];
    if marker == '`' && info.contains('`') {
        return None;
    }
    Some((marker, length, info.trim()))
}

fn is_closing(text: &str, marker: char, length: usize, in_list: bool) -> bool {
    let trimmed = text.trim_start_matches(' ');
    if !in_list && text.len() - trimmed.len() > 3 {
        return false;
    }
    let trimmed = trimmed.trim_start();
    let run = trimmed.chars().take_while(|c| *c == marker).count();
    run >= length && trimmed[run..].trim().is_empty()
}

/// Whether rustdoc compiles a fenced block with this language string (`LangString::parse`): a
/// Rust tag (`should_panic`, `no_run`, `test_harness`, `compile_fail`, `standalone_crate`, an
/// `Ennnn` error code) counts only when no other word came before it, `rust` always counts, an
/// `editionNNNN` is neutral, and any other word makes the block not Rust when no Rust tag stands.
/// A Rust block is compiled unless it is `ignore`. An `ignore-<target>` ignores the block on one
/// target only, so the block is compiled on the others and here counts as compiled.
fn compiled(info: &str) -> bool {
    let mut rust = true;
    let mut ignore = false;
    let mut seen_rust_tags = false;
    let mut seen_other_tags = false;
    for token in info
        .split([',', ' ', '\t'])
        .filter(|token| !token.is_empty())
    {
        match token {
            "should_panic" | "no_run" => seen_rust_tags = !seen_other_tags,
            "ignore" => {
                ignore = true;
                seen_rust_tags = !seen_other_tags;
            }
            "rust" => {
                rust = true;
                seen_rust_tags = true;
            }
            "test_harness" | "compile_fail" | "standalone_crate" => {
                seen_rust_tags = !seen_other_tags || seen_rust_tags;
            }
            _ if token.starts_with("ignore-") => seen_rust_tags = !seen_other_tags,
            _ if token.starts_with("edition") => {}
            _ if token.len() == 5
                && token.starts_with('E')
                && token[1..].bytes().all(|b| b.is_ascii_digit()) =>
            {
                seen_rust_tags = !seen_other_tags || seen_rust_tags;
            }
            _ => seen_other_tags = true,
        }
    }
    rust &= !seen_other_tags || seen_rust_tags;
    rust && !ignore
}

/// A code line with rustdoc's hidden-line prefix removed: `#` alone or `# `/`#\t` before the rest
/// hides the line (the rest is still compiled), and `##` stands for a literal `#`.
fn hidden_stripped(text: &str) -> String {
    let trimmed = text.trim_start();
    let indent = &text[..text.len() - trimmed.len()];
    if let Some(rest) = trimmed.strip_prefix("##") {
        format!("{indent}#{rest}")
    } else if trimmed == "#" {
        String::new()
    } else if let Some(rest) = trimmed
        .strip_prefix("# ")
        .or_else(|| trimmed.strip_prefix("#\t"))
    {
        rest.to_owned()
    } else {
        text.to_owned()
    }
}
