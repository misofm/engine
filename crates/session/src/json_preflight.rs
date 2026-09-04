//! Bounded lexical preflight for invariants the value parser does not expose structurally.

use std::collections::BTreeSet;

use json_syntax::Parse as _;

use crate::{DiagnosticPath, PathSegment};

pub(crate) const MAXIMUM_JSON_DEPTH: usize = 128;

pub(crate) struct SyntaxRefusal {
    pub(crate) path: DiagnosticPath,
    pub(crate) span: core::ops::Range<usize>,
    pub(crate) message: &'static str,
}

pub(crate) fn preflight(source: &str) -> Result<(), SyntaxRefusal> {
    if source.as_bytes().starts_with(&[0xef, 0xbb, 0xbf]) {
        return Err(SyntaxRefusal {
            path: DiagnosticPath::root(),
            span: 0..3,
            message: "UTF-8 BOM is not JSON whitespace",
        });
    }
    let mut scanner = Scanner {
        source,
        cursor: 0,
        path: Vec::new(),
    };
    scanner.whitespace();
    // Malformed JSON is diagnosed by the authoritative dependency. This pass only returns the
    // two refusals for which the value parser does not expose the required contract information.
    scanner.value(1)?;
    Ok(())
}

struct Scanner<'a> {
    source: &'a str,
    cursor: usize,
    path: Vec<PathSegment>,
}

impl Scanner<'_> {
    fn value(&mut self, depth: usize) -> Result<(), SyntaxRefusal> {
        self.whitespace();
        match self.peek() {
            Some(b'{') => self.object(depth),
            Some(b'[') => self.array(depth),
            Some(b'"') => {
                let _ = self.string();
                Ok(())
            }
            Some(_) => {
                self.primitive();
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn object(&mut self, depth: usize) -> Result<(), SyntaxRefusal> {
        self.open(depth)?;
        self.cursor += 1;
        self.whitespace();
        let mut keys = BTreeSet::new();
        if self.peek() == Some(b'}') {
            self.cursor += 1;
            return Ok(());
        }
        loop {
            self.whitespace();
            let key_start = self.cursor;
            let Ok(key_token) = self.string() else {
                return Ok(());
            };
            let key_end = key_start + key_token.len();
            // json-syntax 0.12.5 has no fork-added skip-code-map fast path (`parse_str_value`);
            // `String::parse_str` is the closest equivalent -- it parses just the string token
            // and still builds a throwaway single-entry `CodeMap`, which this preflight scan
            // discards immediately since it only needs the decoded key text.
            let Ok((key, _code_map)) = json_syntax::String::parse_str(key_token) else {
                return Ok(());
            };
            let key = key.to_string();
            if !keys.insert(key.clone()) {
                let mut path = DiagnosticPath::root();
                for segment in &self.path {
                    path = match segment {
                        PathSegment::Field(value) => path.key(value),
                        PathSegment::Index(value) => path.index(*value),
                        PathSegment::Id(_) => unreachable!(),
                    };
                }
                path = path.key(&key);
                return Err(SyntaxRefusal {
                    path,
                    span: key_start..key_end,
                    message: "duplicate object member",
                });
            }
            self.whitespace();
            if self.peek() != Some(b':') {
                return Ok(());
            }
            self.cursor += 1;
            self.path.push(PathSegment::Field(key));
            self.value(depth + 1)?;
            self.path.pop();
            self.whitespace();
            match self.peek() {
                Some(b',') => self.cursor += 1,
                Some(b'}') => {
                    self.cursor += 1;
                    return Ok(());
                }
                _ => return Ok(()),
            }
        }
    }

    fn array(&mut self, depth: usize) -> Result<(), SyntaxRefusal> {
        self.open(depth)?;
        self.cursor += 1;
        self.whitespace();
        if self.peek() == Some(b']') {
            self.cursor += 1;
            return Ok(());
        }
        let mut index = 0;
        loop {
            self.path.push(PathSegment::Index(index));
            self.value(depth + 1)?;
            self.path.pop();
            index += 1;
            self.whitespace();
            match self.peek() {
                Some(b',') => self.cursor += 1,
                Some(b']') => {
                    self.cursor += 1;
                    return Ok(());
                }
                _ => return Ok(()),
            }
        }
    }

    fn open(&self, depth: usize) -> Result<(), SyntaxRefusal> {
        if depth <= MAXIMUM_JSON_DEPTH {
            return Ok(());
        }
        let mut path = DiagnosticPath::root();
        for segment in &self.path {
            path = match segment {
                PathSegment::Field(value) => path.key(value),
                PathSegment::Index(value) => path.index(*value),
                PathSegment::Id(_) => unreachable!(),
            };
        }
        Err(SyntaxRefusal {
            path,
            span: self.cursor..self.cursor + 1,
            message: "JSON nesting exceeds the maximum depth of 128",
        })
    }

    fn string(&mut self) -> Result<&str, ()> {
        let start = self.cursor;
        if self.peek() != Some(b'"') {
            return Err(());
        }
        self.cursor += 1;
        while let Some(byte) = self.peek() {
            match byte {
                b'"' => {
                    self.cursor += 1;
                    return Ok(&self.source[start..self.cursor]);
                }
                b'\\' => {
                    self.cursor += 1;
                    if self.peek().is_none() {
                        return Err(());
                    }
                    self.cursor += 1;
                }
                0..=0x1f => return Err(()),
                _ => self.cursor += 1,
            }
        }
        Err(())
    }

    fn primitive(&mut self) {
        while !matches!(
            self.peek(),
            None | Some(b' ' | b'\t' | b'\r' | b'\n' | b',' | b']' | b'}')
        ) {
            self.cursor += 1;
        }
    }
    fn whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\r' | b'\n')) {
            self.cursor += 1;
        }
    }
    fn peek(&self) -> Option<u8> {
        self.source.as_bytes().get(self.cursor).copied()
    }
}
