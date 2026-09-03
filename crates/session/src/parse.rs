//! Explicit typed walk over a strict, spanned JSON value tree.
use crate::{
    Automation, AutomationSegment, AutomationTarget, ChannelBuiltins, ChannelMatrix, Diagnostic,
    DiagnosticCode, DiagnosticPath as OwnedDiagnosticPath, DiagnosticSet, DualMonoBuiltins,
    DualMonoFader, Effect, EffectIdentity, EffectParam, MatrixOrPan, Output, OutputProfile, Rack,
    RenderProfile, Route, RouteDestination, RouteSource, SESSION_SCHEMA_VERSION_V1, SessionModel,
    Sidechain, SidechainDeclaration, Source, SourceBitDepth, SourceSpan, StableId, Submix, Track,
    diagnostic::{MAXIMUM_SESSION_DIAGNOSTICS, PathRef as DiagnosticPath, PathSegment},
    json_preflight,
    model::ClosedToken,
    validate::validate_session,
    value::{F32Token, parse_f32_token},
};
use core::ops::Range;
use jstrict::{CodeMap, Object, Parse, Value, array::JsonArray};
fn code_span(code_map: &CodeMap, offset: usize) -> (usize, usize) {
    let span = code_map[offset].span;
    (span.start, span.end)
}
#[derive(Clone, Copy)]
struct ValueRef<'v> {
    value: &'v Value,
    offset: usize,
    span: (usize, usize),
}
impl<'v> ValueRef<'v> {
    fn get_ref(self) -> &'v Value {
        self.value
    }
    fn span(self) -> (usize, usize) {
        self.span
    }
    fn start(self) -> usize {
        self.span.0
    }
    fn end(self) -> usize {
        self.span.1
    }
}
fn value_span(value: ValueRef<'_>) -> (usize, usize) {
    value.span()
}
#[derive(Clone, Copy)]
struct TableRef<'v> {
    table: &'v Object,
    offset: usize,
    span: (usize, usize),
}
struct ArrayRef<'v> {
    values: &'v [Value],
    offset: usize,
}

struct Parser<'i> {
    source: &'i str,
    code_map: &'i CodeMap,
    diagnostics: Vec<Diagnostic>,
}

impl<'i> Parser<'i> {
    fn new(source: &'i str, code_map: &'i CodeMap) -> Self {
        Self {
            source,
            code_map,
            diagnostics: Vec::new(),
        }
    }

    fn value<'v>(&self, table: TableRef<'v>, key: &str) -> Option<ValueRef<'v>> {
        table
            .table
            .get_mapped(self.code_map, table.offset, key)
            .next()
            .map(|mapped| ValueRef {
                value: mapped.value,
                offset: mapped.offset,
                span: code_span(self.code_map, mapped.offset),
            })
    }

    fn error_at(
        &mut self,
        code: DiagnosticCode,
        path: DiagnosticPath<'_>,
        span: (usize, usize),
        message: impl Into<String>,
    ) {
        if self.diagnostics.len() >= MAXIMUM_SESSION_DIAGNOSTICS {
            return;
        }
        self.diagnostics.push(Diagnostic::at(
            code,
            &path,
            Some(SourceSpan::from_range(self.source, span.0..span.1)),
            message,
        ));
    }

    fn keys(&mut self, table: TableRef<'_>, allowed: &[&str], path: &DiagnosticPath<'_>) {
        for entry in table.table.iter_mapped(self.code_map, table.offset) {
            let key = entry.value.key;
            let name = key.value.as_str();
            if !allowed.contains(&name) {
                let span = code_span(self.code_map, key.offset);
                self.error_at(
                    DiagnosticCode::UnknownField,
                    path.key(name),
                    span,
                    "key is not part of SESSION_SCHEMA_VERSION_V1",
                );
            }
        }
    }

    fn reject_key(
        &mut self,
        table: TableRef<'_>,
        key: &str,
        path: DiagnosticPath<'_>,
        message: &str,
    ) {
        if let Some(actual) = table
            .table
            .get_mapped_entries(self.code_map, table.offset, key)
            .next()
        {
            let span = code_span(self.code_map, actual.value.key.offset);
            self.error_at(DiagnosticCode::UnknownField, path, span, message);
        }
    }

    fn error_field(
        &mut self,
        table: TableRef<'_>,
        key: &str,
        path: &DiagnosticPath<'_>,
        code: DiagnosticCode,
        message: &str,
    ) {
        let span = self.value(table, key).map_or(table.span, ValueRef::span);
        self.error_at(code, path.key(key), span, message);
    }

    fn missing<T>(
        &mut self,
        table: TableRef<'_>,
        key: &str,
        path: &DiagnosticPath<'_>,
    ) -> Option<T> {
        self.error_at(
            DiagnosticCode::MissingField,
            path.key(key),
            table.span,
            "required key is absent",
        );
        None
    }

    fn table_value<'v>(
        &mut self,
        input: ValueRef<'v>,
        path: DiagnosticPath<'_>,
    ) -> Option<TableRef<'v>> {
        let value = input;
        match value.get_ref() {
            Value::Object(table) => Some(TableRef {
                table,
                offset: value.offset,
                span: value.span(),
            }),
            _ => {
                self.error_at(
                    DiagnosticCode::WrongType,
                    path,
                    value.span(),
                    "expected JSON object",
                );
                None
            }
        }
    }

    fn table<'v>(
        &mut self,
        table: TableRef<'v>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<TableRef<'v>> {
        let Some(value) = self.value(table, key) else {
            return self.missing(table, key, path);
        };
        self.table_value(value, path.key(key))
    }

    fn array<'v>(
        &mut self,
        table: TableRef<'v>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<ArrayRef<'v>> {
        let Some(value) = self.value(table, key) else {
            return self.missing(table, key, path);
        };
        match value.get_ref() {
            Value::Array(array) => Some(ArrayRef {
                values: array,
                offset: value.offset,
            }),
            _ => {
                self.error_at(
                    DiagnosticCode::WrongType,
                    path.key(key),
                    value_span(value),
                    "expected JSON array",
                );
                None
            }
        }
    }

    fn string_ref<'v>(
        &mut self,
        table: TableRef<'v>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<&'v str> {
        let Some(value) = self.value(table, key) else {
            return self.missing(table, key, path);
        };
        match value.get_ref() {
            Value::String(value) => Some(value.as_str()),
            _ => {
                self.error_at(
                    DiagnosticCode::WrongType,
                    path.key(key),
                    value_span(value),
                    "expected JSON string",
                );
                None
            }
        }
    }

    fn string(
        &mut self,
        table: TableRef<'_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<String> {
        self.string_ref(table, key, path).map(str::to_owned)
    }

    fn bool(
        &mut self,
        table: TableRef<'_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<bool> {
        let Some(value) = self.value(table, key) else {
            return self.missing(table, key, path);
        };
        match value.get_ref() {
            Value::Boolean(value) => Some(*value),
            _ => {
                self.error_at(
                    DiagnosticCode::WrongType,
                    path.key(key),
                    value_span(value),
                    "expected JSON boolean",
                );
                None
            }
        }
    }

    fn u64(
        &mut self,
        table: TableRef<'_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<u64> {
        let Some(value) = self.value(table, key) else {
            return self.missing(table, key, path);
        };
        let span = value_span(value);
        match value.get_ref() {
            Value::String(text) if is_canonical_u64(text) => text.parse().ok().or_else(|| {
                self.error_at(
                    DiagnosticCode::NumericOutOfSchemaRange,
                    path.key(key),
                    span,
                    "unsigned decimal string exceeds u64",
                );
                None
            }),
            Value::String(_) => {
                self.error_at(
                    DiagnosticCode::NumericOutOfSchemaRange,
                    path.key(key),
                    span,
                    "expected a canonical unsigned decimal string",
                );
                None
            }
            _ => {
                self.error_at(
                    DiagnosticCode::WrongType,
                    path.key(key),
                    span,
                    "expected JSON string for u64 field",
                );
                None
            }
        }
    }

    fn unsigned_number(
        &mut self,
        table: TableRef<'_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<u64> {
        let Some(value) = self.value(table, key) else {
            return self.missing(table, key, path);
        };
        let span = value_span(value);
        match value.get_ref() {
            Value::Number(_) => {
                let token = &self.source[span.0..span.1];
                if token.starts_with('-') || token.contains(['.', 'e', 'E']) {
                    self.error_at(
                        DiagnosticCode::NumericOutOfSchemaRange,
                        path.key(key),
                        span,
                        "expected an unsigned integer JSON number",
                    );
                    None
                } else {
                    token.parse().ok().or_else(|| {
                        self.error_at(
                            DiagnosticCode::NumericOutOfSchemaRange,
                            path.key(key),
                            span,
                            "integer exceeds u64",
                        );
                        None
                    })
                }
            }
            _ => {
                self.error_at(
                    DiagnosticCode::WrongType,
                    path.key(key),
                    span,
                    "expected JSON number",
                );
                None
            }
        }
    }

    fn u32(
        &mut self,
        table: TableRef<'_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<u32> {
        let value = self.unsigned_number(table, key, path)?;
        u32::try_from(value).ok().or_else(|| {
            self.error_field(
                table,
                key,
                path,
                DiagnosticCode::NumericOutOfSchemaRange,
                "integer must fit u32",
            );
            None
        })
    }

    fn u8(
        &mut self,
        table: TableRef<'_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<u8> {
        let value = self.unsigned_number(table, key, path)?;
        u8::try_from(value).ok().or_else(|| {
            self.error_field(
                table,
                key,
                path,
                DiagnosticCode::NumericOutOfSchemaRange,
                "integer must fit u8",
            );
            None
        })
    }

    fn source_bit_depth(
        &mut self,
        table: TableRef<'_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<SourceBitDepth> {
        let Some(value) = self.value(table, key) else {
            return self.missing(table, key, path);
        };
        let parsed = match value.get_ref() {
            Value::Number(_) if &self.source[value.start()..value.end()] == "16" => {
                Some(SourceBitDepth::Pcm16)
            }
            Value::Number(_) if &self.source[value.start()..value.end()] == "24" => {
                Some(SourceBitDepth::Pcm24)
            }
            Value::String(token) if token == "32f" => Some(SourceBitDepth::Float32),
            _ => None,
        };
        if parsed.is_none() {
            self.error_at(
                DiagnosticCode::SourceBitDepthUnsupported,
                path.key(key),
                value_span(value),
                "source bit_depth must be 16, 24, or \"32f\"",
            );
        }
        parsed
    }

    fn f32(
        &mut self,
        table: TableRef<'_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<f32> {
        let Some(value) = self.value(table, key) else {
            return self.missing(table, key, path);
        };
        let field_path = path.key(key);
        let span = value_span(value);
        match value.get_ref() {
            Value::Number(_) => match parse_f32_token(&self.source[span.0..span.1]) {
                F32Token::Value(value) => Some(value),
                F32Token::NonFinite => {
                    self.error_at(
                        DiagnosticCode::NumericNonFinite,
                        field_path,
                        span,
                        "value must be finite",
                    );
                    None
                }
                F32Token::NotRepresentable => {
                    self.error_at(
                        DiagnosticCode::NumericNotF32Representable,
                        field_path,
                        span,
                        "value must be representable as f32",
                    );
                    None
                }
            },
            _ => {
                self.error_at(
                    DiagnosticCode::WrongType,
                    field_path,
                    span,
                    "expected JSON number",
                );
                None
            }
        }
    }

    fn id(
        &mut self,
        table: TableRef<'_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<StableId> {
        let field_path = path.key(key);
        let span = self.value(table, key).map(value_span).unwrap_or(table.span);
        let value = self.string_ref(table, key, path)?;
        StableId::parse(value).or_else(|| {
            self.error_at(
                DiagnosticCode::InvalidId,
                field_path,
                span,
                "stable IDs must match [a-z][a-z0-9._-]{0,126}",
            );
            None
        })
    }

    fn token<'v>(
        &mut self,
        table: TableRef<'v>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<&'v str> {
        let Some(value) = self.value(table, key) else {
            return self.missing(table, key, path);
        };
        match value.get_ref() {
            Value::String(value) => Some(value.as_str()),
            _ => {
                self.error_at(
                    DiagnosticCode::WrongType,
                    path.key(key),
                    value_span(value),
                    "expected JSON string",
                );
                None
            }
        }
    }

    fn closed_token<T: ClosedToken>(
        &mut self,
        table: TableRef<'_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
        code: DiagnosticCode,
    ) -> Option<T> {
        let token = self.token(table, key, path)?;
        if let Some((value, _)) = T::ALL.iter().find(|(_, candidate)| *candidate == token) {
            return Some(*value);
        }
        let mut allowed = String::new();
        for (_, token) in T::ALL {
            if !allowed.is_empty() {
                allowed.push_str(", ");
            }
            allowed.push_str(token);
        }
        self.error_field(
            table,
            key,
            path,
            code,
            &format!("expected one of: {allowed}"),
        );
        None
    }

    fn bounded(
        &mut self,
        table: TableRef<'_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
        minimum: f32,
        maximum: f32,
    ) -> Option<f32> {
        let value = self.f32(table, key, path)?;
        if value < minimum || value > maximum {
            self.error_field(
                table,
                key,
                path,
                DiagnosticCode::NumericOutOfSchemaRange,
                &format!("value must be in [{minimum}, {maximum}]"),
            );
            None
        } else {
            Some(value)
        }
    }
}

/// Parse strict JSON into the V1 typed model and validate it.
pub fn parse_session_json(source: &str) -> Result<SessionModel, DiagnosticSet> {
    if let Err(refusal) = json_preflight::preflight(source) {
        return Err(DiagnosticSet::from_vec(vec![Diagnostic::new(
            DiagnosticCode::JsonSyntax,
            refusal.path,
            Some(SourceSpan::from_range(source, refusal.span)),
            refusal.message,
        )]));
    }
    let (root, code_map) = match Value::parse_str(source) {
        Ok(parsed) => parsed,
        Err(error) => {
            let span = error.span();
            return Err(DiagnosticSet::from_vec(vec![Diagnostic::new(
                DiagnosticCode::JsonSyntax,
                OwnedDiagnosticPath::root(),
                Some(SourceSpan::from_range(source, span.range())),
                error.to_string(),
            )]));
        }
    };
    let mut parser = Parser::new(source, &code_map);
    let root_path = DiagnosticPath::root();
    let Some(object) = root.as_object() else {
        return Err(DiagnosticSet::from_vec(vec![Diagnostic::new(
            DiagnosticCode::WrongType,
            OwnedDiagnosticPath::root(),
            Some(SourceSpan::from_range(source, code_map[0].span.range())),
            "expected JSON object",
        )]));
    };
    let root_table = TableRef {
        table: object,
        offset: 0,
        span: code_span(&code_map, 0),
    };
    let model = parse_root(&mut parser, root_table, root_path);
    if parser.diagnostics.is_empty() {
        match model {
            Some(model) => match validate_session(&model) {
                Ok(()) => Ok(model),
                Err(errors) => Err(DiagnosticSet::from_vec(
                    errors
                        .diagnostics()
                        .iter()
                        .map(|diagnostic| {
                            let range = span_for_path(&root, &code_map, &diagnostic.path)
                                .unwrap_or_else(|| code_map[0].span.range());
                            Diagnostic::new(
                                diagnostic.code,
                                diagnostic.path.clone(),
                                Some(SourceSpan::from_range(source, range)),
                                diagnostic.message.clone(),
                            )
                        })
                        .collect(),
                )),
            },
            None => Err(DiagnosticSet::from_vec(vec![Diagnostic::new(
                DiagnosticCode::WrongType,
                OwnedDiagnosticPath::root(),
                Some(SourceSpan::whole(source)),
                "parser could not build a typed model",
            )])),
        }
    } else {
        Err(DiagnosticSet::from_vec(parser.diagnostics))
    }
}

fn span_for_path(
    root: &Value,
    code_map: &CodeMap,
    path: &OwnedDiagnosticPath,
) -> Option<Range<usize>> {
    let mut node = root;
    let mut offset = 0;
    let mut resolved = code_map[0].span.range();
    for segment in path.segments() {
        let next = match (segment, node) {
            (PathSegment::Field(field), Value::Object(table)) => {
                table.get_mapped(code_map, offset, field.as_str()).next()
            }
            (PathSegment::Index(index), Value::Array(array)) => {
                array.iter_mapped(code_map, offset).nth(*index)
            }
            _ => None,
        };
        let Some(mapped) = next else {
            break;
        };
        offset = mapped.offset;
        resolved = code_map[offset].span.range();
        node = mapped.value;
    }
    Some(resolved)
}

fn is_canonical_u64(text: &str) -> bool {
    text == "0"
        || (!text.is_empty()
            && !text.starts_with('0')
            && text.bytes().all(|byte| byte.is_ascii_digit()))
}

fn parse_root(
    parser: &mut Parser<'_>,
    table: TableRef<'_>,
    path: DiagnosticPath<'_>,
) -> Option<SessionModel> {
    let schema_path = path.key("schema_version");
    if !table.table.contains_key("schema_version") {
        parser.error_at(
            DiagnosticCode::VersionMissing,
            schema_path,
            table.span,
            "required schema version is absent",
        );
        return None;
    }
    let schema_version = parser.u32(table, "schema_version", &path)?;
    if schema_version != SESSION_SCHEMA_VERSION_V1 {
        let span = parser
            .value(table, "schema_version")
            .map_or(table.span, ValueRef::span);
        parser.error_at(
            DiagnosticCode::VersionUnsupported,
            schema_path,
            span,
            "only version 1 is accepted",
        );
        return None;
    }
    parser.keys(
        table,
        &[
            "schema_version",
            "session_id",
            "revision",
            "sample_rate_hz",
            "quantum_frames",
            "render_profile",
            "output_profile",
            "sources",
            "tracks",
            "submixes",
            "outputs",
            "routes",
            "automation",
        ],
        &path,
    );
    let schema_version = Some(schema_version);
    let session_id = parser.id(table, "session_id", &path);
    let revision = parser.u64(table, "revision", &path);
    let sample_rate_hz = parser.u32(table, "sample_rate_hz", &path);
    let quantum_frames = parser.u32(table, "quantum_frames", &path);
    let render_profile = parse_record(parser, table, "render_profile", &path, parse_render_profile);
    let output_profile = parse_record(parser, table, "output_profile", &path, parse_output_profile);
    let sources = parse_list(parser, table, "sources", &path, parse_source);
    let tracks = parse_list(parser, table, "tracks", &path, parse_track);
    let submixes = parse_list(parser, table, "submixes", &path, parse_submix);
    let outputs = parse_list(parser, table, "outputs", &path, parse_output);
    let routes = parse_list(parser, table, "routes", &path, parse_route);
    let automation = parse_list(parser, table, "automation", &path, parse_automation);
    match (
        schema_version,
        session_id,
        revision,
        sample_rate_hz,
        quantum_frames,
        render_profile,
        output_profile,
        sources,
        tracks,
        submixes,
        outputs,
        routes,
        automation,
    ) {
        (
            Some(schema_version),
            Some(session_id),
            Some(revision),
            Some(sample_rate_hz),
            Some(quantum_frames),
            Some(render_profile),
            Some(output_profile),
            Some(sources),
            Some(tracks),
            Some(submixes),
            Some(outputs),
            Some(routes),
            Some(automation),
        ) => Some(SessionModel {
            schema_version,
            session_id,
            revision,
            sample_rate_hz,
            quantum_frames,
            render_profile,
            output_profile,
            sources,
            tracks,
            submixes,
            outputs,
            routes,
            automation,
        }),
        _ => None,
    }
}

fn parse_render_profile(
    parser: &mut Parser,
    table: TableRef<'_>,
    path: DiagnosticPath,
) -> Option<RenderProfile> {
    parser.keys(table, &["id", "mode"], &path);
    let id = parser.id(table, "id", &path);
    let mode = parser.closed_token(table, "mode", &path, DiagnosticCode::InvalidEnum);
    Some(RenderProfile {
        id: id?,
        mode: mode?,
    })
}

fn parse_output_profile(
    parser: &mut Parser,
    table: TableRef<'_>,
    path: DiagnosticPath,
) -> Option<OutputProfile> {
    parser.keys(table, &["id", "channels", "sample_format"], &path);
    let id = parser.id(table, "id", &path);
    let channels = parser.u8(table, "channels", &path);
    let sample_format =
        parser.closed_token(table, "sample_format", &path, DiagnosticCode::InvalidEnum);
    Some(OutputProfile {
        id: id?,
        channels: channels?,
        sample_format: sample_format?,
    })
}

fn parse_record<T>(
    parser: &mut Parser,
    parent: TableRef<'_>,
    key: &'static str,
    path: &DiagnosticPath,
    parse: fn(&mut Parser, TableRef<'_>, DiagnosticPath) -> Option<T>,
) -> Option<T> {
    let child_path = path.key(key);
    let table = parser.table(parent, key, path)?;
    parse(parser, table, child_path)
}

fn parse_list<T>(
    parser: &mut Parser,
    parent: TableRef<'_>,
    key: &'static str,
    path: &DiagnosticPath,
    parse: fn(&mut Parser, TableRef<'_>, DiagnosticPath) -> Option<T>,
) -> Option<Vec<T>> {
    let list_path = path.key(key);
    let values = parser.array(parent, key, path)?;
    let mut output = Vec::with_capacity(values.values.len());
    for (index, mapped) in values
        .values
        .iter_mapped(parser.code_map, values.offset)
        .enumerate()
    {
        let value = ValueRef {
            value: mapped.value,
            offset: mapped.offset,
            span: code_span(parser.code_map, mapped.offset),
        };
        let item_path = list_path.index(index);
        let Some(table) = parser.table_value(value, item_path) else {
            continue;
        };
        if let Some(item) = parse(parser, table, item_path) {
            output.push(item);
        }
    }
    Some(output)
}

fn parse_source(parser: &mut Parser, table: TableRef<'_>, path: DiagnosticPath) -> Option<Source> {
    parser.keys(
        table,
        &["id", "content", "channels", "bit_depth", "frames"],
        &path,
    );
    let id = parser.id(table, "id", &path);
    let content = parser.string(table, "content", &path);
    let channels = parser.u8(table, "channels", &path);
    let bit_depth = parser.source_bit_depth(table, "bit_depth", &path);
    let frames = parser.u64(table, "frames", &path);
    Some(Source {
        id: id?,
        content: content?,
        channels: channels?,
        bit_depth: bit_depth?,
        frames: frames?,
    })
}

fn parse_track(parser: &mut Parser, table: TableRef<'_>, path: DiagnosticPath) -> Option<Track> {
    parser.keys(
        table,
        &[
            "id",
            "source_id",
            "left_source_channel",
            "right_source_channel",
            "builtins",
            "simd1",
            "dynamic",
            "simd2",
            "fader",
            "pan",
            "matrix",
        ],
        &path,
    );
    let id = parser.id(table, "id", &path);
    let source_id = parser.id(table, "source_id", &path);
    let left_source_channel = parser.u8(table, "left_source_channel", &path);
    let right_source_channel = parser.u8(table, "right_source_channel", &path);
    let builtins = parse_record(parser, table, "builtins", &path, parse_builtins);
    let simd1 = parse_record(parser, table, "simd1", &path, parse_rack);
    let dynamic = parse_record(parser, table, "dynamic", &path, parse_rack);
    let simd2 = parse_record(parser, table, "simd2", &path, parse_rack);
    let fader = parse_record(parser, table, "fader", &path, parse_fader);
    let matrix_or_pan = parse_matrix_or_pan(parser, table, &path);
    Some(Track {
        id: id?,
        source_id: source_id?,
        left_source_channel: left_source_channel?,
        right_source_channel: right_source_channel?,
        builtins: builtins?,
        simd1: simd1?,
        dynamic: dynamic?,
        simd2: simd2?,
        fader: fader?,
        matrix_or_pan: matrix_or_pan?,
    })
}

fn parse_builtins(
    parser: &mut Parser,
    table: TableRef<'_>,
    path: DiagnosticPath,
) -> Option<DualMonoBuiltins> {
    parser.keys(table, &["left", "right"], &path);
    let left = parse_record(parser, table, "left", &path, parse_channel_builtins);
    let right = parse_record(parser, table, "right", &path, parse_channel_builtins);
    Some(DualMonoBuiltins {
        left: left?,
        right: right?,
    })
}

fn parse_channel_builtins(
    parser: &mut Parser,
    table: TableRef<'_>,
    path: DiagnosticPath,
) -> Option<ChannelBuiltins> {
    parser.keys(
        table,
        &[
            "polarity_invert",
            "trim_db",
            "hpf_hz",
            "lpf_hz",
            "delay_samples",
        ],
        &path,
    );
    let polarity_invert = parser.bool(table, "polarity_invert", &path);
    let trim_db = parser.f32(table, "trim_db", &path);
    let hpf_hz = parser.f32(table, "hpf_hz", &path);
    let lpf_hz = parser.f32(table, "lpf_hz", &path);
    // Required, like every other V1 key: the schema has no optional fields and no unknown-key
    // tolerance, so an existing document without it is a `schema.missing_field`, not a default.
    let delay_samples = parser.u32(table, "delay_samples", &path);
    Some(ChannelBuiltins {
        polarity_invert: polarity_invert?,
        trim_db: trim_db?,
        hpf_hz: hpf_hz?,
        lpf_hz: lpf_hz?,
        delay_samples: delay_samples?,
    })
}

fn parse_rack(parser: &mut Parser, table: TableRef<'_>, path: DiagnosticPath) -> Option<Rack> {
    parser.keys(table, &["effects"], &path);
    let effects = parse_list(parser, table, "effects", &path, parse_effect);
    Some(Rack { effects: effects? })
}

fn parse_effect(parser: &mut Parser, table: TableRef<'_>, path: DiagnosticPath) -> Option<Effect> {
    parser.keys(
        table,
        &[
            "id",
            "identity",
            "quality",
            "bypass",
            "link_mode",
            "params",
            "sidechain",
        ],
        &path,
    );
    let id = parser.id(table, "id", &path);
    let identity = parse_record(parser, table, "identity", &path, parse_effect_identity);
    let quality = parser.closed_token(table, "quality", &path, DiagnosticCode::InvalidEnum);
    let bypass = parser.bool(table, "bypass", &path);
    let link_mode = parser.closed_token(table, "link_mode", &path, DiagnosticCode::InvalidEnum);
    let params = parse_list(parser, table, "params", &path, parse_param);
    let sidechain = parse_record(parser, table, "sidechain", &path, parse_sidechain);
    Some(Effect {
        id: id?,
        identity: identity?,
        quality: quality?,
        bypass: bypass?,
        link_mode: link_mode?,
        params: params?,
        sidechain: sidechain?,
    })
}

fn parse_effect_identity(
    parser: &mut Parser,
    table: TableRef<'_>,
    path: DiagnosticPath,
) -> Option<EffectIdentity> {
    parser.keys(table, &["kind", "effect_id", "cid"], &path);
    match parser.token(table, "kind", &path)? {
        "native" => {
            parser.reject_key(
                table,
                "cid",
                path.key("cid"),
                "native effect identity cannot contain cid",
            );
            let effect_id = parser.id(table, "effect_id", &path);
            Some(EffectIdentity::Native {
                effect_id: effect_id?,
            })
        }
        "cid" => {
            parser.reject_key(
                table,
                "effect_id",
                path.key("effect_id"),
                "CID effect identity cannot contain effect_id",
            );
            let cid = parser.string(table, "cid", &path);
            Some(EffectIdentity::ThirdPartyCid { cid: cid? })
        }
        _ => {
            parser.error_field(
                table,
                "kind",
                &path,
                DiagnosticCode::InvalidEnum,
                "expected native or cid",
            );
            None
        }
    }
}

fn parse_param(
    parser: &mut Parser,
    table: TableRef<'_>,
    path: DiagnosticPath,
) -> Option<EffectParam> {
    parser.keys(table, &["parameter_id", "channel", "unit", "value"], &path);
    let parameter_id = parser.u32(table, "parameter_id", &path);
    let channel = parser.closed_token(table, "channel", &path, DiagnosticCode::InvalidEnum);
    let unit = parser.closed_token(table, "unit", &path, DiagnosticCode::UnitInvalid);
    let value = parser.f32(table, "value", &path);
    Some(EffectParam {
        parameter_id: parameter_id?,
        channel: channel?,
        unit: unit?,
        value: value?,
    })
}

fn parse_sidechain(
    parser: &mut Parser,
    table: TableRef<'_>,
    path: DiagnosticPath,
) -> Option<SidechainDeclaration> {
    parser.keys(table, &["kind", "source", "port_id"], &path);
    match parser.token(table, "kind", &path)? {
        "none" => {
            for key in ["source", "port_id"] {
                parser.reject_key(
                    table,
                    key,
                    path.key(key),
                    "none sidechain cannot contain routing fields",
                );
            }
            Some(SidechainDeclaration::None)
        }
        "routed" => Some(SidechainDeclaration::Routed(Sidechain {
            source: parse_record(parser, table, "source", &path, parse_route_source)?,
            port_id: parser.id(table, "port_id", &path)?,
        })),
        _ => {
            parser.error_field(
                table,
                "kind",
                &path,
                DiagnosticCode::InvalidEnum,
                "expected none or routed",
            );
            None
        }
    }
}

fn parse_fader(
    parser: &mut Parser,
    table: TableRef<'_>,
    path: DiagnosticPath,
) -> Option<DualMonoFader> {
    parser.keys(
        table,
        &["left_db", "right_db", "left_mute", "right_mute"],
        &path,
    );
    let left_db = parser.f32(table, "left_db", &path);
    let right_db = parser.f32(table, "right_db", &path);
    let left_mute = parser.bool(table, "left_mute", &path);
    let right_mute = parser.bool(table, "right_mute", &path);
    Some(DualMonoFader {
        left_db: left_db?,
        right_db: right_db?,
        left_mute: left_mute?,
        right_mute: right_mute?,
    })
}

fn parse_matrix_or_pan(
    parser: &mut Parser,
    table: TableRef<'_>,
    path: &DiagnosticPath,
) -> Option<MatrixOrPan> {
    match (parser.value(table, "pan"), parser.value(table, "matrix")) {
        (Some(_), Some(_)) => {
            parser.error_at(
                DiagnosticCode::WrongType,
                *path,
                table.span,
                "exactly one of pan or matrix is required",
            );
            None
        }
        (None, None) => {
            parser.error_at(
                DiagnosticCode::MissingField,
                *path,
                table.span,
                "exactly one of pan or matrix is required",
            );
            None
        }
        (Some(value), None) => {
            let pan_path = path.key("pan");
            let pan = parser.table_value(value, pan_path)?;
            parser.keys(pan, &["left", "right", "smoothing_samples"], &pan_path);
            let left = parser.bounded(pan, "left", &pan_path, -1.0, 1.0);
            let right = parser.bounded(pan, "right", &pan_path, -1.0, 1.0);
            let smoothing_samples = parser.u32(pan, "smoothing_samples", &pan_path);
            Some(MatrixOrPan::Pan {
                left: left?,
                right: right?,
                smoothing_samples: smoothing_samples?,
            })
        }
        (None, Some(value)) => {
            let matrix_path = path.key("matrix");
            let matrix = parser.table_value(value, matrix_path)?;
            parser.keys(
                matrix,
                &["ll", "lr", "rl", "rr", "smoothing_samples"],
                &matrix_path,
            );
            let ll = parser.f32(matrix, "ll", &matrix_path);
            let lr = parser.f32(matrix, "lr", &matrix_path);
            let rl = parser.f32(matrix, "rl", &matrix_path);
            let rr = parser.f32(matrix, "rr", &matrix_path);
            let smoothing_samples = parser.u32(matrix, "smoothing_samples", &matrix_path);
            Some(MatrixOrPan::Matrix {
                ll: ll?,
                lr: lr?,
                rl: rl?,
                rr: rr?,
                smoothing_samples: smoothing_samples?,
            })
        }
    }
}

fn parse_submix(parser: &mut Parser, table: TableRef<'_>, path: DiagnosticPath) -> Option<Submix> {
    parser.keys(table, &["id"], &path);
    Some(Submix {
        id: parser.id(table, "id", &path)?,
    })
}

fn parse_output(parser: &mut Parser, table: TableRef<'_>, path: DiagnosticPath) -> Option<Output> {
    parser.keys(table, &["id"], &path);
    Some(Output {
        id: parser.id(table, "id", &path)?,
    })
}

fn parse_route(parser: &mut Parser, table: TableRef<'_>, path: DiagnosticPath) -> Option<Route> {
    parser.keys(
        table,
        &["id", "source", "destination", "channel_matrix", "gain_db"],
        &path,
    );
    let id = parser.id(table, "id", &path);
    let source = parse_record(parser, table, "source", &path, parse_route_source);
    let destination = parse_record(parser, table, "destination", &path, parse_route_destination);
    let channel_matrix = parse_record(parser, table, "channel_matrix", &path, parse_channel_matrix);
    let gain_db = parser.f32(table, "gain_db", &path);
    Some(Route {
        id: id?,
        source: source?,
        destination: destination?,
        channel_matrix: channel_matrix?,
        gain_db: gain_db?,
    })
}

fn parse_route_source(
    parser: &mut Parser,
    table: TableRef<'_>,
    path: DiagnosticPath,
) -> Option<RouteSource> {
    parser.keys(table, &["kind", "track_id", "tap", "submix_id"], &path);
    match parser.token(table, "kind", &path)? {
        "track" => {
            parser.reject_key(
                table,
                "submix_id",
                path.key("submix_id"),
                "track source cannot contain submix_id",
            );
            Some(RouteSource::Track {
                track_id: parser.id(table, "track_id", &path)?,
                tap: parser.closed_token(table, "tap", &path, DiagnosticCode::InvalidEnum)?,
            })
        }
        "submix_output" => {
            for key in ["track_id", "tap"] {
                parser.reject_key(
                    table,
                    key,
                    path.key(key),
                    "submix_output source cannot contain track fields",
                );
            }
            Some(RouteSource::SubmixOutput {
                submix_id: parser.id(table, "submix_id", &path)?,
            })
        }
        _ => {
            parser.error_field(
                table,
                "kind",
                &path,
                DiagnosticCode::InvalidEnum,
                "expected track or submix_output",
            );
            None
        }
    }
}

fn parse_route_destination(
    parser: &mut Parser,
    table: TableRef<'_>,
    path: DiagnosticPath,
) -> Option<RouteDestination> {
    parser.keys(table, &["kind", "submix_id", "output_id"], &path);
    match parser.token(table, "kind", &path)? {
        "submix_input" => {
            parser.reject_key(
                table,
                "output_id",
                path.key("output_id"),
                "submix_input cannot contain output_id",
            );
            Some(RouteDestination::SubmixInput {
                submix_id: parser.id(table, "submix_id", &path)?,
            })
        }
        "output_input" => {
            parser.reject_key(
                table,
                "submix_id",
                path.key("submix_id"),
                "output_input cannot contain submix_id",
            );
            Some(RouteDestination::OutputInput {
                output_id: parser.id(table, "output_id", &path)?,
            })
        }
        _ => {
            parser.error_field(
                table,
                "kind",
                &path,
                DiagnosticCode::InvalidEnum,
                "expected submix_input or output_input",
            );
            None
        }
    }
}

fn parse_channel_matrix(
    parser: &mut Parser,
    table: TableRef<'_>,
    path: DiagnosticPath,
) -> Option<ChannelMatrix> {
    parser.keys(table, &["ll", "lr", "rl", "rr"], &path);
    let ll = parser.f32(table, "ll", &path);
    let lr = parser.f32(table, "lr", &path);
    let rl = parser.f32(table, "rl", &path);
    let rr = parser.f32(table, "rr", &path);
    Some(ChannelMatrix {
        ll: ll?,
        lr: lr?,
        rl: rl?,
        rr: rr?,
    })
}

fn parse_automation(
    parser: &mut Parser,
    table: TableRef<'_>,
    path: DiagnosticPath,
) -> Option<Automation> {
    parser.keys(table, &["id", "target", "segments"], &path);
    let id = parser.id(table, "id", &path);
    let target = parse_record(parser, table, "target", &path, parse_target);
    let segments = parse_list(parser, table, "segments", &path, parse_segment);
    Some(Automation {
        id: id?,
        target: target?,
        segments: segments?,
    })
}

fn parse_target(
    parser: &mut Parser,
    table: TableRef<'_>,
    path: DiagnosticPath,
) -> Option<AutomationTarget> {
    parser.keys(
        table,
        &["entity_id", "rack", "effect_id", "parameter_id", "channel"],
        &path,
    );
    let entity_id = parser.id(table, "entity_id", &path);
    let rack = parser.closed_token(table, "rack", &path, DiagnosticCode::InvalidEnum);
    let effect_id = parser.id(table, "effect_id", &path);
    let parameter_id = parser.u32(table, "parameter_id", &path);
    let channel = parser.closed_token(table, "channel", &path, DiagnosticCode::InvalidEnum);
    Some(AutomationTarget {
        entity_id: entity_id?,
        rack: rack?,
        effect_id: effect_id?,
        parameter_id: parameter_id?,
        channel: channel?,
    })
}

fn parse_segment(
    parser: &mut Parser,
    table: TableRef<'_>,
    path: DiagnosticPath,
) -> Option<AutomationSegment> {
    parser.keys(
        table,
        &[
            "shape",
            "start_sample",
            "end_sample",
            "start_value",
            "end_value",
            "unit",
        ],
        &path,
    );
    let shape = parser.closed_token(table, "shape", &path, DiagnosticCode::InvalidEnum);
    let start_sample = parser.u64(table, "start_sample", &path);
    let end_sample = parser.u64(table, "end_sample", &path);
    let start_value = parser.f32(table, "start_value", &path);
    let end_value = parser.f32(table, "end_value", &path);
    let unit = parser.closed_token(table, "unit", &path, DiagnosticCode::UnitInvalid);
    Some(AutomationSegment {
        shape: shape?,
        start_sample: start_sample?,
        end_sample: end_sample?,
        start_value: start_value?,
        end_value: end_value?,
        unit: unit?,
    })
}
