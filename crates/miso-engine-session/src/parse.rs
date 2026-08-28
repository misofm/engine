//! Explicit value-walking parser over `toml::de::DeTable` (borrowed, spanned). Serde is not used.
use crate::{
    Automation, AutomationSegment, AutomationTarget, ChannelBuiltins, ChannelMatrix, Diagnostic,
    DiagnosticCode, DiagnosticPath as OwnedDiagnosticPath, DiagnosticSet, DualMonoBuiltins,
    DualMonoFader, Effect, EffectIdentity, EffectParam, MatrixOrPan, Output, OutputProfile, Rack,
    RenderProfile, Route, RouteDestination, RouteSource, SESSION_SCHEMA_VERSION_V1, SessionLimits,
    SessionToml, Sidechain, SidechainDeclaration, Source, SourceContent, SourceMapping,
    SourceRegion, SourceSpan, StableId, Submix, Track,
    diagnostic::{PathRef as DiagnosticPath, PathSegment},
    model::ClosedToken,
    validate::validate_session,
    value::{F32Token, parse_f32_token, parse_i64_token},
};
use core::ops::Range;
use toml::{
    Spanned,
    de::{DeTable, DeValue},
};
type Value<'i> = Spanned<DeValue<'i>>;
fn value_span(value: &Value<'_>) -> (usize, usize) {
    let span = value.span();
    (span.start, span.end)
}
#[derive(Clone, Copy)]
struct TableRef<'v, 'i> {
    table: &'v DeTable<'i>,
    span: (usize, usize),
}

type ValueRef<'v, 'i> = &'v Value<'i>;

struct Parser<'i> {
    source: &'i str,
    diagnostics: Vec<Diagnostic>,
}

impl<'i> Parser<'i> {
    fn new(source: &'i str) -> Self {
        Self {
            source,
            diagnostics: Vec::new(),
        }
    }

    fn error_at(
        &mut self,
        code: DiagnosticCode,
        path: DiagnosticPath<'_>,
        span: (usize, usize),
        message: impl Into<String>,
    ) {
        self.diagnostics.push(Diagnostic::at(
            code,
            &path,
            Some(SourceSpan::from_range(self.source, span.0..span.1)),
            message,
        ));
    }

    fn keys(&mut self, table: TableRef<'_, '_>, allowed: &[&str], path: &DiagnosticPath<'_>) {
        for key in table.table.keys() {
            let name = key.get_ref().as_ref();
            if !allowed.contains(&name) {
                let span = key.span();
                self.error_at(
                    DiagnosticCode::UnknownField,
                    path.key(name),
                    (span.start, span.end),
                    "key is not part of SESSION_SCHEMA_VERSION_V1",
                );
            }
        }
    }

    fn reject_key(
        &mut self,
        table: TableRef<'_, '_>,
        key: &str,
        path: DiagnosticPath<'_>,
        message: &str,
    ) {
        if let Some((actual, _)) = table
            .table
            .iter()
            .find(|(actual, _)| actual.get_ref().as_ref() == key)
        {
            let span = actual.span();
            self.error_at(
                DiagnosticCode::UnknownField,
                path,
                (span.start, span.end),
                message,
            );
        }
    }

    fn error_field(
        &mut self,
        table: TableRef<'_, '_>,
        key: &str,
        path: &DiagnosticPath<'_>,
        code: DiagnosticCode,
        message: &str,
    ) {
        let span = table.table.get(key).map_or(table.span, |value| {
            let span = value.span();
            (span.start, span.end)
        });
        self.error_at(code, path.key(key), span, message);
    }

    fn missing<T>(
        &mut self,
        table: TableRef<'_, '_>,
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

    fn table_value<'v, 'd>(
        &mut self,
        input: ValueRef<'v, 'd>,
        path: DiagnosticPath<'_>,
    ) -> Option<TableRef<'v, 'd>> {
        let value = input;
        match value.get_ref() {
            DeValue::Table(table) => {
                let span = value.span();
                Some(TableRef {
                    table,
                    span: (span.start, span.end),
                })
            }
            _ => {
                let span = value.span();
                self.error_at(
                    DiagnosticCode::WrongType,
                    path,
                    (span.start, span.end),
                    "expected TOML table",
                );
                None
            }
        }
    }

    fn table<'v, 'd>(
        &mut self,
        table: TableRef<'v, 'd>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<TableRef<'v, 'd>> {
        let Some(value) = table.table.get(key) else {
            return self.missing(table, key, path);
        };
        self.table_value(value, path.key(key))
    }

    fn array<'v, 'd>(
        &mut self,
        table: TableRef<'v, 'd>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<&'v [Value<'d>]> {
        let Some(value) = table.table.get(key) else {
            return self.missing(table, key, path);
        };
        match value.get_ref() {
            DeValue::Array(array) => Some(array),
            _ => {
                self.error_at(
                    DiagnosticCode::WrongType,
                    path.key(key),
                    value_span(value),
                    "expected TOML array",
                );
                None
            }
        }
    }

    fn string_ref<'v, 'd>(
        &mut self,
        table: TableRef<'v, 'd>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<&'v str> {
        let Some(value) = table.table.get(key) else {
            return self.missing(table, key, path);
        };
        match value.get_ref() {
            DeValue::String(value) => Some(value.as_ref()),
            _ => {
                self.error_at(
                    DiagnosticCode::WrongType,
                    path.key(key),
                    value_span(value),
                    "expected TOML string",
                );
                None
            }
        }
    }

    fn string(
        &mut self,
        table: TableRef<'_, '_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<String> {
        self.string_ref(table, key, path).map(str::to_owned)
    }

    fn bool(
        &mut self,
        table: TableRef<'_, '_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<bool> {
        let Some(value) = table.table.get(key) else {
            return self.missing(table, key, path);
        };
        match value.get_ref() {
            DeValue::Boolean(value) => Some(*value),
            _ => {
                self.error_at(
                    DiagnosticCode::WrongType,
                    path.key(key),
                    value_span(value),
                    "expected TOML boolean",
                );
                None
            }
        }
    }

    fn u64(
        &mut self,
        table: TableRef<'_, '_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<u64> {
        let Some(value) = table.table.get(key) else {
            return self.missing(table, key, path);
        };
        let span = value_span(value);
        match value.get_ref() {
            DeValue::Integer(integer) => match parse_i64_token(integer.as_str(), integer.radix()) {
                Some(value) if value >= 0 => Some(value as u64),
                Some(_) => {
                    self.error_at(
                        DiagnosticCode::NumericOutOfSchemaRange,
                        path.key(key),
                        span,
                        "expected a non-negative integer",
                    );
                    None
                }
                None => {
                    self.error_at(
                        DiagnosticCode::NumericOutOfSchemaRange,
                        path.key(key),
                        span,
                        "integer exceeds the TOML i64 range",
                    );
                    None
                }
            },
            _ => {
                self.error_at(
                    DiagnosticCode::WrongType,
                    path.key(key),
                    span,
                    "expected TOML integer",
                );
                None
            }
        }
    }

    fn u32(
        &mut self,
        table: TableRef<'_, '_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<u32> {
        let value = self.u64(table, key, path)?;
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
        table: TableRef<'_, '_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<u8> {
        let value = self.u64(table, key, path)?;
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

    fn f32(
        &mut self,
        table: TableRef<'_, '_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<f32> {
        let Some(value) = table.table.get(key) else {
            return self.missing(table, key, path);
        };
        let field_path = path.key(key);
        let span = value_span(value);
        match value.get_ref() {
            DeValue::Float(float) => match parse_f32_token(float.as_str()) {
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
            DeValue::Integer(integer) => match parse_i64_token(integer.as_str(), integer.radix()) {
                Some(value) => Some(value as f32),
                None => {
                    self.error_at(
                        DiagnosticCode::NumericOutOfSchemaRange,
                        field_path,
                        span,
                        "integer exceeds the TOML i64 range",
                    );
                    None
                }
            },
            _ => {
                self.error_at(
                    DiagnosticCode::WrongType,
                    field_path,
                    span,
                    "expected TOML integer or float",
                );
                None
            }
        }
    }

    fn id(
        &mut self,
        table: TableRef<'_, '_>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<StableId> {
        let field_path = path.key(key);
        let span = table.table.get(key).map(value_span).unwrap_or(table.span);
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

    fn token<'v, 'd>(
        &mut self,
        table: TableRef<'v, 'd>,
        key: &'static str,
        path: &DiagnosticPath<'_>,
    ) -> Option<&'v str> {
        let Some(value) = table.table.get(key) else {
            return self.missing(table, key, path);
        };
        match value.get_ref() {
            DeValue::String(value) => Some(value.as_ref()),
            _ => {
                self.error_at(
                    DiagnosticCode::WrongType,
                    path.key(key),
                    value_span(value),
                    "expected TOML string",
                );
                None
            }
        }
    }

    fn closed_token<T: ClosedToken>(
        &mut self,
        table: TableRef<'_, '_>,
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
        table: TableRef<'_, '_>,
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

/// Parse TOML text (`toml_parser` 1.1 grammar) into the V1 typed model and validate it.
pub fn parse_session_toml(source: &str) -> Result<SessionToml, DiagnosticSet> {
    let root = match DeTable::parse(source) {
        Ok(value) => value,
        Err(error) => {
            let range = error.span().unwrap_or(0..source.len());
            return Err(DiagnosticSet::from_vec(vec![Diagnostic::new(
                DiagnosticCode::TomlSyntax,
                OwnedDiagnosticPath::root(),
                Some(SourceSpan::from_range(source, range)),
                error.to_string(),
            )]));
        }
    };
    let mut parser = Parser::new(source);
    let root_path = DiagnosticPath::root();
    let root_span = root.span();
    let root_table = TableRef {
        table: root.get_ref(),
        span: (root_span.start, root_span.end),
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
                            let range = span_for_path(&root, &diagnostic.path)
                                .unwrap_or_else(|| root.span());
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

fn span_for_path(root: &Spanned<DeTable<'_>>, path: &OwnedDiagnosticPath) -> Option<Range<usize>> {
    enum Node<'a, 'i> {
        Table(&'a DeTable<'i>),
        Value(&'a Value<'i>),
    }

    let mut node = Node::Table(root.get_ref());
    let mut resolved = root.span();
    for segment in path.segments() {
        let next = match (segment, node) {
            (PathSegment::Field(field), Node::Table(table)) => table.get(field.as_str()),
            (PathSegment::Field(field), Node::Value(value)) => match value.get_ref() {
                DeValue::Table(table) => table.get(field.as_str()),
                _ => None,
            },
            (PathSegment::Index(index), Node::Value(value)) => match value.get_ref() {
                DeValue::Array(array) => array.get(*index),
                _ => None,
            },
            (PathSegment::Id(_), _) | (PathSegment::Index(_), Node::Table(_)) => None,
        };
        let Some(value) = next else {
            break;
        };
        resolved = value.span();
        node = Node::Value(value);
    }
    Some(resolved)
}

fn parse_root(
    parser: &mut Parser<'_>,
    table: TableRef<'_, '_>,
    path: DiagnosticPath<'_>,
) -> Option<SessionToml> {
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
        let span = table
            .table
            .get("schema_version")
            .map_or(table.span, |value| {
                let span = value.span();
                (span.start, span.end)
            });
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
            "limits",
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
    let limits = parse_record(parser, table, "limits", &path, parse_limits);
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
        limits,
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
            Some(limits),
            Some(sources),
            Some(tracks),
            Some(submixes),
            Some(outputs),
            Some(routes),
            Some(automation),
        ) => Some(SessionToml {
            schema_version,
            session_id,
            revision,
            sample_rate_hz,
            quantum_frames,
            render_profile,
            output_profile,
            limits,
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
    table: TableRef<'_, '_>,
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
    table: TableRef<'_, '_>,
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

fn parse_limits(
    parser: &mut Parser,
    table: TableRef<'_, '_>,
    path: DiagnosticPath,
) -> Option<SessionLimits> {
    parser.keys(
        table,
        &["pcm_ring_frames", "control_queue_messages", "memory_bytes"],
        &path,
    );
    let pcm_ring_frames = parser.u64(table, "pcm_ring_frames", &path);
    let control_queue_messages = parser.u64(table, "control_queue_messages", &path);
    let memory_bytes = parser.u64(table, "memory_bytes", &path);
    Some(SessionLimits {
        pcm_ring_frames: pcm_ring_frames?,
        control_queue_messages: control_queue_messages?,
        memory_bytes: memory_bytes?,
    })
}

fn parse_record<T>(
    parser: &mut Parser,
    parent: TableRef<'_, '_>,
    key: &'static str,
    path: &DiagnosticPath,
    parse: fn(&mut Parser, TableRef<'_, '_>, DiagnosticPath) -> Option<T>,
) -> Option<T> {
    let child_path = path.key(key);
    let table = parser.table(parent, key, path)?;
    parse(parser, table, child_path)
}

fn parse_list<T>(
    parser: &mut Parser,
    parent: TableRef<'_, '_>,
    key: &'static str,
    path: &DiagnosticPath,
    parse: fn(&mut Parser, TableRef<'_, '_>, DiagnosticPath) -> Option<T>,
) -> Option<Vec<T>> {
    let list_path = path.key(key);
    let values = parser.array(parent, key, path)?;
    let mut output = Vec::with_capacity(values.len());
    for (index, value) in values.iter().enumerate() {
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

fn parse_source(
    parser: &mut Parser,
    table: TableRef<'_, '_>,
    path: DiagnosticPath,
) -> Option<Source> {
    parser.keys(
        table,
        &["id", "sample_rate_hz", "content", "mapping"],
        &path,
    );
    let id = parser.id(table, "id", &path);
    let sample_rate_hz = parser.u32(table, "sample_rate_hz", &path);
    let content = parse_record(parser, table, "content", &path, parse_source_content);
    let mapping = parse_record(parser, table, "mapping", &path, parse_source_mapping);
    Some(Source {
        id: id?,
        sample_rate_hz: sample_rate_hz?,
        content: content?,
        mapping: mapping?,
    })
}

fn parse_source_content(
    parser: &mut Parser,
    table: TableRef<'_, '_>,
    path: DiagnosticPath,
) -> Option<SourceContent> {
    parser.keys(table, &["identity", "locator"], &path);
    let identity = parser.string(table, "identity", &path);
    let locator = parser.string(table, "locator", &path);
    Some(SourceContent {
        identity: identity?,
        locator: locator?,
    })
}

fn parse_source_mapping(
    parser: &mut Parser,
    table: TableRef<'_, '_>,
    path: DiagnosticPath,
) -> Option<SourceMapping> {
    parser.keys(table, &["channel_count", "region"], &path);
    let channel_count = parser.u8(table, "channel_count", &path);
    let region = parse_record(parser, table, "region", &path, parse_region);
    Some(SourceMapping {
        channel_count: channel_count?,
        region: region?,
    })
}

fn parse_region(
    parser: &mut Parser,
    table: TableRef<'_, '_>,
    path: DiagnosticPath,
) -> Option<SourceRegion> {
    parser.keys(table, &["start_sample", "length_samples"], &path);
    let start_sample = parser.u64(table, "start_sample", &path);
    let length_samples = parser.u64(table, "length_samples", &path);
    Some(SourceRegion {
        start_sample: start_sample?,
        length_samples: length_samples?,
    })
}

fn parse_track(
    parser: &mut Parser,
    table: TableRef<'_, '_>,
    path: DiagnosticPath,
) -> Option<Track> {
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
    table: TableRef<'_, '_>,
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
    table: TableRef<'_, '_>,
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

fn parse_rack(parser: &mut Parser, table: TableRef<'_, '_>, path: DiagnosticPath) -> Option<Rack> {
    parser.keys(table, &["effects"], &path);
    let effects = parse_list(parser, table, "effects", &path, parse_effect);
    Some(Rack { effects: effects? })
}

fn parse_effect(
    parser: &mut Parser,
    table: TableRef<'_, '_>,
    path: DiagnosticPath,
) -> Option<Effect> {
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
    table: TableRef<'_, '_>,
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
    table: TableRef<'_, '_>,
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
    table: TableRef<'_, '_>,
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
    table: TableRef<'_, '_>,
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
    table: TableRef<'_, '_>,
    path: &DiagnosticPath,
) -> Option<MatrixOrPan> {
    match (table.table.get("pan"), table.table.get("matrix")) {
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

fn parse_submix(
    parser: &mut Parser,
    table: TableRef<'_, '_>,
    path: DiagnosticPath,
) -> Option<Submix> {
    parser.keys(table, &["id"], &path);
    Some(Submix {
        id: parser.id(table, "id", &path)?,
    })
}

fn parse_output(
    parser: &mut Parser,
    table: TableRef<'_, '_>,
    path: DiagnosticPath,
) -> Option<Output> {
    parser.keys(table, &["id"], &path);
    Some(Output {
        id: parser.id(table, "id", &path)?,
    })
}

fn parse_route(
    parser: &mut Parser,
    table: TableRef<'_, '_>,
    path: DiagnosticPath,
) -> Option<Route> {
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
    table: TableRef<'_, '_>,
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
    table: TableRef<'_, '_>,
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
    table: TableRef<'_, '_>,
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
    table: TableRef<'_, '_>,
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
    table: TableRef<'_, '_>,
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
    table: TableRef<'_, '_>,
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
