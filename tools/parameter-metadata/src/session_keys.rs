//! Build-time transcription of the session JSON's key tables (issue #1569).
//!
//! # Why this document exists
//!
//! The SDK writes a session as canonical JSON, byte for byte as the Rust writer does. That needs
//! three facts per record that no JavaScript object carries: the order of its keys, which of its
//! numbers are integers and which are floats (`0` versus `0.0`), and which record a nested object
//! is. Before this document the SDK held those facts in a hand copy that "transcribed the Rust
//! field walk", and a new session field had to be added in two languages.
//!
//! The Rust field walk (`session::VisitModel`) already knows all three. This module runs it over
//! every document of the canonical writer corpus with a recording visitor and writes down what it
//! saw: for each `(key module, variant arm)` the keys in emit order with their kinds, and for each
//! parent JSON key the records that may sit under it.
//!
//! # The generator refuses gaps
//!
//! A table made from a corpus is only as complete as the corpus. [`Recorder::finish`] therefore
//! fails, naming what is missing, when a variant arm is never walked, when a key of a key module is
//! never walked, when one `(module, arm)` is walked with two different key lists, when one numeric
//! leaf name is walked as two kinds, and when two records that share a parent and a key set differ
//! in key order or kinds (the SDK picks a record by its key set, so a tie is allowed only when the
//! writer's answer is the same either way). The fix for a gap is a corpus document written by the
//! Rust canonical writer, never a hand edit here.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use session::{
    FieldKey, ModelVisitor, SessionModel, SourceBitDepth, StableId, Token, VARIANT_ARMS,
    VisitModel, WalkOrder, keys, parse_session_json,
};

/// The emitted file name. It is an SDK source asset, not a member of the release artifact set.
pub const OUTPUT_NAME: &str = "miso-engine-v1-session-keys.json";
/// The schema tag every consumer must check before reading a field.
pub const SCHEMA: &str = "miso.session.keys.v1";
/// The canonical writer corpus the walk runs over, relative to the repository root.
pub const CORPUS_PATH: &str = "fixtures/session-canonical/v1/canonical-writer-corpus.json";

/// What a walked key holds.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Kind {
    /// `u8`, `u32` or the mixed bit-depth token: a JSON number spelled without a fraction.
    Integer,
    /// `u64`: a JSON string of decimal digits.
    U64,
    /// `f32`: a JSON number spelled with a fraction.
    Float,
    /// A Boolean.
    Boolean,
    /// A stable ID string.
    Id,
    /// Arbitrary text.
    Text,
    /// A closed token string.
    Token,
    /// A nested record.
    Record,
    /// An array.
    Array,
}

impl Kind {
    /// The spelling in the emitted document.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Integer => "integer",
            Self::U64 => "u64",
            Self::Float => "float",
            Self::Boolean => "boolean",
            Self::Id => "id",
            Self::Text => "text",
            Self::Token => "token",
            Self::Record => "record",
            Self::Array => "array",
        }
    }
}

/// One record shape: a key module and, for a variant record, its arm.
type Shape = (String, Option<String>);
type KeyList = Vec<(String, Kind)>;

enum Frame {
    Record {
        parent: String,
        shape: Option<Shape>,
        keys: KeyList,
    },
    Array {
        key: String,
    },
}

/// A visitor that records every record's keys in emit order.
#[derive(Default)]
pub struct Recorder {
    stack: Vec<Frame>,
    records: BTreeMap<Shape, KeyList>,
    parents: BTreeMap<String, BTreeSet<Shape>>,
    numeric: BTreeMap<String, Kind>,
}

impl Recorder {
    fn push_key(&mut self, name: &str, kind: Kind) -> Result<(), String> {
        match self.stack.last_mut() {
            Some(Frame::Record { keys, .. }) => {
                keys.push((name.to_owned(), kind));
                Ok(())
            }
            _ => Err(format!("key `{name}` was walked outside a record")),
        }
    }

    fn leaf(&mut self, key: FieldKey, kind: Kind) -> Result<(), String> {
        if matches!(kind, Kind::Integer | Kind::Float | Kind::U64) {
            match self.numeric.get(key.name) {
                Some(seen) if *seen != kind => {
                    return Err(format!(
                        "the numeric key `{}` was walked as both {} and {}",
                        key.name,
                        seen.name(),
                        kind.name()
                    ));
                }
                _ => {
                    self.numeric.insert(key.name.to_owned(), kind);
                }
            }
        }
        self.push_key(key.name, kind)
    }

    /// Check the walk is complete and consistent, then render the document.
    ///
    /// # Errors
    /// Names the first gap or conflict (see the module documentation).
    pub fn finish(self) -> Result<String, String> {
        if !self.stack.is_empty() {
            return Err("the walk ended inside a record or array".to_owned());
        }
        for (module, arm) in VARIANT_ARMS {
            if !self
                .records
                .contains_key(&((*module).to_owned(), Some((*arm).to_owned())))
            {
                return Err(format!(
                    "the variant arm {module}/{arm} is never walked; add a corpus document that reaches it"
                ));
            }
        }
        for ((module, arm), key_list) in &self.records {
            let Some((_, fields)) = keys::MODULES.iter().find(|(name, _)| name == module) else {
                return Err(format!("`{module}` is not a key module"));
            };
            for (name, _) in key_list {
                if !fields.iter().any(|field| field.name == name) {
                    return Err(format!(
                        "the walk emitted `{name}` in {} but the key module does not declare it",
                        shape_text(module, arm.as_deref())
                    ));
                }
            }
        }
        for (module, fields) in keys::MODULES {
            for field in *fields {
                let walked = self.records.iter().any(|((name, _), key_list)| {
                    name == module && key_list.iter().any(|(key, _)| key == field.name)
                });
                if !walked {
                    return Err(format!(
                        "the key `{module}.{}` is never walked; add a corpus document that reaches it",
                        field.name
                    ));
                }
            }
        }
        self.check_ties()?;
        Ok(self.render())
    }

    /// Check that records sharing a parent key and a key set agree on key order and kinds.
    ///
    /// The SDK picks a record by its key set; a tie is harmless only when every candidate would
    /// write the same bytes.
    ///
    /// # Errors
    /// Names the parent key and the differing records.
    pub fn check_ties(&self) -> Result<(), String> {
        for (parent, shapes) in &self.parents {
            let mut by_set: BTreeMap<Vec<&str>, Vec<&Shape>> = BTreeMap::new();
            for shape in shapes {
                let mut names: Vec<&str> = self.records[shape]
                    .iter()
                    .map(|(name, _)| name.as_str())
                    .collect();
                names.sort_unstable();
                by_set.entry(names).or_default().push(shape);
            }
            for tied in by_set.values().filter(|tied| tied.len() > 1) {
                let first = &self.records[tied[0]];
                if tied.iter().any(|shape| &self.records[*shape] != first) {
                    let names: Vec<String> = tied
                        .iter()
                        .map(|(module, arm)| shape_text(module, arm.as_deref()))
                        .collect();
                    return Err(format!(
                        "under the parent key `{parent}` the records {} share a key set but differ in key order or kinds",
                        names.join(" and ")
                    ));
                }
            }
        }
        Ok(())
    }

    fn render(&self) -> String {
        let module_index = |module: &str| {
            keys::MODULES
                .iter()
                .position(|(name, _)| *name == module)
                .unwrap_or(usize::MAX)
        };
        let mut shapes: Vec<&Shape> = self.records.keys().collect();
        shapes.sort_by(|a, b| (module_index(&a.0), &a.1).cmp(&(module_index(&b.0), &b.1)));
        let mut out = String::new();
        out.push_str("{\n");
        append!(&mut out, "  \"schema\": \"{SCHEMA}\",\n");
        out.push_str("  \"records\": [\n");
        for (index, shape) in shapes.iter().enumerate() {
            out.push_str("    {\n");
            append!(&mut out, "      \"module\": \"{}\",\n", shape.0);
            match &shape.1 {
                Some(arm) => append!(&mut out, "      \"arm\": \"{arm}\",\n"),
                None => out.push_str("      \"arm\": null,\n"),
            }
            out.push_str("      \"keys\": [\n");
            let key_list = &self.records[*shape];
            for (position, (name, kind)) in key_list.iter().enumerate() {
                let comma = if position + 1 < key_list.len() {
                    ","
                } else {
                    ""
                };
                append!(
                    &mut out,
                    "        [\"{name}\", \"{}\"]{comma}\n",
                    kind.name()
                );
            }
            out.push_str("      ]\n");
            let comma = if index + 1 < shapes.len() { "," } else { "" };
            append!(&mut out, "    }}{comma}\n");
        }
        out.push_str("  ],\n  \"parents\": [\n");
        for (index, (parent, shapes)) in self.parents.iter().enumerate() {
            out.push_str("    {\n");
            append!(&mut out, "      \"key\": \"{parent}\",\n");
            out.push_str("      \"records\": [\n");
            let mut ordered: Vec<&Shape> = shapes.iter().collect();
            ordered.sort_by(|a, b| (module_index(&a.0), &a.1).cmp(&(module_index(&b.0), &b.1)));
            for (position, (module, arm)) in
                ordered.iter().map(|shape| (&shape.0, &shape.1)).enumerate()
            {
                let comma = if position + 1 < ordered.len() {
                    ","
                } else {
                    ""
                };
                match arm {
                    Some(arm) => append!(
                        &mut out,
                        "        {{ \"module\": \"{module}\", \"arm\": \"{arm}\" }}{comma}\n"
                    ),
                    None => append!(
                        &mut out,
                        "        {{ \"module\": \"{module}\", \"arm\": null }}{comma}\n"
                    ),
                }
            }
            out.push_str("      ]\n");
            let comma = if index + 1 < self.parents.len() {
                ","
            } else {
                ""
            };
            append!(&mut out, "    }}{comma}\n");
        }
        out.push_str("  ]\n}\n");
        out
    }
}

fn shape_text(module: &str, arm: Option<&str>) -> String {
    arm.map_or_else(|| module.to_owned(), |arm| format!("{module}/{arm}"))
}

impl ModelVisitor for Recorder {
    type Error = String;

    fn record_begin(&mut self, key: Option<FieldKey>, _wire_fields: u32) -> Result<(), String> {
        let parent = match (key, self.stack.last()) {
            (Some(key), _) => key.name.to_owned(),
            (None, Some(Frame::Array { key })) => key.clone(),
            (None, None) => String::new(),
            (None, Some(Frame::Record { .. })) => {
                return Err("a record without a key sits directly in a record".to_owned());
            }
        };
        if let Some(key) = key {
            self.push_key(key.name, Kind::Record)?;
        }
        self.stack.push(Frame::Record {
            parent,
            shape: None,
            keys: Vec::new(),
        });
        Ok(())
    }

    fn record_schema(
        &mut self,
        module: &'static str,
        arm: Option<&'static str>,
    ) -> Result<(), String> {
        match self.stack.last_mut() {
            Some(Frame::Record { shape, .. }) if shape.is_none() => {
                *shape = Some((module.to_owned(), arm.map(str::to_owned)));
                Ok(())
            }
            _ => Err(format!("record_schema({module}) outside a fresh record")),
        }
    }

    fn record_end(&mut self) -> Result<(), String> {
        let Some(Frame::Record {
            parent,
            shape,
            keys: key_list,
        }) = self.stack.pop()
        else {
            return Err("record_end without a record".to_owned());
        };
        let shape = shape.ok_or("a record was walked without record_schema")?;
        match self.records.get(&shape) {
            Some(seen) if *seen != key_list => {
                return Err(format!(
                    "{} was walked with two different key lists: [{}] and [{}]",
                    shape_text(&shape.0, shape.1.as_deref()),
                    key_text(seen),
                    key_text(&key_list)
                ));
            }
            Some(_) => {}
            None => {
                self.records.insert(shape.clone(), key_list);
            }
        }
        self.parents.entry(parent).or_default().insert(shape);
        Ok(())
    }

    fn array_begin(&mut self, key: FieldKey, _len: usize) -> Result<(), String> {
        self.push_key(key.name, Kind::Array)?;
        self.stack.push(Frame::Array {
            key: key.name.to_owned(),
        });
        Ok(())
    }

    fn array_end(&mut self) -> Result<(), String> {
        match self.stack.pop() {
            Some(Frame::Array { .. }) => Ok(()),
            _ => Err("array_end without an array".to_owned()),
        }
    }

    fn wire_tag(&mut self, _tag: Token) -> Result<(), String> {
        Ok(())
    }

    fn bool(&mut self, key: FieldKey, _value: bool) -> Result<(), String> {
        self.leaf(key, Kind::Boolean)
    }

    fn u8(&mut self, key: FieldKey, _value: u8) -> Result<(), String> {
        self.leaf(key, Kind::Integer)
    }

    fn u32(&mut self, key: FieldKey, _value: u32) -> Result<(), String> {
        self.leaf(key, Kind::Integer)
    }

    fn u64(&mut self, key: FieldKey, _value: u64) -> Result<(), String> {
        self.leaf(key, Kind::U64)
    }

    fn source_bit_depth(&mut self, key: FieldKey, _value: SourceBitDepth) -> Result<(), String> {
        self.leaf(key, Kind::Integer)
    }

    fn f32(&mut self, key: FieldKey, _value: f32) -> Result<(), String> {
        self.leaf(key, Kind::Float)
    }

    fn id(&mut self, key: FieldKey, _value: &StableId) -> Result<(), String> {
        self.leaf(key, Kind::Id)
    }

    fn id_item(&mut self, _value: &StableId) -> Result<(), String> {
        Ok(())
    }

    fn text(&mut self, key: FieldKey, _value: &str) -> Result<(), String> {
        self.leaf(key, Kind::Text)
    }

    fn token(&mut self, key: FieldKey, _value: Token) -> Result<(), String> {
        self.leaf(key, Kind::Token)
    }
}

fn key_text(key_list: &KeyList) -> String {
    key_list
        .iter()
        .map(|(name, kind)| format!("{name}:{}", kind.name()))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Walk every model in canonical order and render the key tables.
///
/// # Errors
/// A gap or conflict, as [`Recorder::finish`] reports it, or a failed walk.
pub fn build(models: &[SessionModel]) -> Result<String, String> {
    let mut recorder = Recorder::default();
    for model in models {
        model.visit(WalkOrder::Canonical, &mut recorder)?;
    }
    recorder.finish()
}

/// Every document of the canonical writer corpus, parsed.
///
/// # Errors
/// An unreadable corpus, or a document that does not parse.
pub fn load_corpus(repository_root: &Path) -> Result<Vec<SessionModel>, String> {
    let path = repository_root.join(CORPUS_PATH);
    let text = std::fs::read_to_string(&path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    let manifest: serde_json::Value =
        serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))?;
    let documents = manifest["documents"]
        .as_array()
        .ok_or("the corpus has no `documents` array")?;
    documents
        .iter()
        .map(|entry| {
            let source = match (entry["path"].as_str(), entry["canonical"].as_str()) {
                (Some(relative), _) => std::fs::read_to_string(repository_root.join(relative))
                    .map_err(|error| format!("cannot read corpus document {relative}: {error}"))?,
                (None, Some(canonical)) => canonical.to_owned(),
                (None, None) => {
                    return Err("a corpus document has neither path nor canonical".to_owned());
                }
            };
            parse_session_json(&source)
                .map_err(|_| format!("corpus document {} does not parse", entry["id"]))
        })
        .collect()
}

/// The document made from the checked-in corpus.
///
/// # Panics
/// With the generator's message when the corpus leaves a gap: the build must not proceed on a
/// partial table.
#[must_use]
pub fn render() -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    load_corpus(&root)
        .and_then(|models| build(&models))
        .unwrap_or_else(|error| panic!("session key tables: {error}"))
}
