//! Explicit TOML 1.0 value-walking parser. Serde is intentionally not used for schema dispatch.

use std::cell::RefCell;

use toml::{Table, Value};

use crate::{
    Automation, AutomationSegment, AutomationShape, AutomationTarget, ChannelBuiltins,
    ChannelMatrix, Diagnostic, DiagnosticCode, DiagnosticPath, DiagnosticSet, DualMonoBuiltins,
    DualMonoFader, Effect, EffectIdentity, EffectParam, EffectQuality, LinkMode, MatrixOrPan,
    Output, OutputProfile, ParameterChannel, ParameterUnit, Rack, RackName, RenderMode,
    RenderProfile, Route, RouteDestination, RouteSource, SESSION_SCHEMA_VERSION_V1, SampleFormat,
    SendTap, SessionLimits, SessionTomlV1, Sidechain, SidechainDeclaration, Source, SourceContent,
    SourceMapping, SourceRegion, SourceSpan, StableId, Submix, Track,
    value::{bounded_f32, f32_value},
};

struct Parser {
    span: SourceSpan,
    diagnostics: RefCell<Vec<Diagnostic>>,
}

impl Parser {
    fn new(source: &str) -> Self {
        Self {
            span: SourceSpan::whole(source),
            diagnostics: RefCell::new(Vec::new()),
        }
    }

    fn error(&self, code: DiagnosticCode, path: DiagnosticPath, message: impl Into<String>) {
        self.diagnostics
            .borrow_mut()
            .push(Diagnostic::new(code, path, Some(self.span), message));
    }

    fn keys(&self, table: &Table, allowed: &[&str], path: &DiagnosticPath) {
        for key in table.keys() {
            if !allowed.contains(&key.as_str()) {
                self.error(
                    DiagnosticCode::UnknownField,
                    path.key(key),
                    "key is not part of SESSION_SCHEMA_VERSION_V1",
                );
            }
        }
    }

    fn required<'a>(
        &self,
        table: &'a Table,
        key: &str,
        path: &DiagnosticPath,
    ) -> Option<&'a Value> {
        match table.get(key) {
            Some(value) => Some(value),
            None => {
                self.error(
                    DiagnosticCode::MissingField,
                    path.key(key),
                    "required key is absent",
                );
                None
            }
        }
    }

    fn optional<'a>(&self, table: &'a Table, key: &str) -> Option<&'a Value> {
        table.get(key)
    }

    fn table<'a>(&self, value: Option<&'a Value>, path: DiagnosticPath) -> Option<&'a Table> {
        match value.and_then(Value::as_table) {
            Some(table) => Some(table),
            None => {
                self.error(DiagnosticCode::WrongType, path, "expected TOML table");
                None
            }
        }
    }

    fn array<'a>(&self, value: Option<&'a Value>, path: DiagnosticPath) -> Option<&'a Vec<Value>> {
        match value.and_then(Value::as_array) {
            Some(array) => Some(array),
            None => {
                self.error(DiagnosticCode::WrongType, path, "expected TOML array");
                None
            }
        }
    }

    fn string(&self, value: Option<&Value>, path: DiagnosticPath) -> Option<String> {
        match value.and_then(Value::as_str) {
            Some(value) => Some(value.to_owned()),
            None => {
                self.error(DiagnosticCode::WrongType, path, "expected TOML string");
                None
            }
        }
    }

    fn bool(&self, value: Option<&Value>, path: DiagnosticPath) -> Option<bool> {
        match value.and_then(Value::as_bool) {
            Some(value) => Some(value),
            None => {
                self.error(DiagnosticCode::WrongType, path, "expected TOML boolean");
                None
            }
        }
    }

    fn u64(&self, value: Option<&Value>, path: DiagnosticPath) -> Option<u64> {
        match value.and_then(Value::as_integer) {
            Some(value) if value >= 0 => Some(value as u64),
            Some(_) => {
                self.error(
                    DiagnosticCode::NumericOutOfSchemaRange,
                    path,
                    "expected a non-negative integer",
                );
                None
            }
            None => {
                self.error(DiagnosticCode::WrongType, path, "expected TOML integer");
                None
            }
        }
    }

    fn u32(&self, value: Option<&Value>, path: DiagnosticPath) -> Option<u32> {
        let value = self.u64(value, path.clone())?;
        match u32::try_from(value) {
            Ok(value) => Some(value),
            Err(_) => {
                self.error(
                    DiagnosticCode::NumericOutOfSchemaRange,
                    path,
                    "integer must fit u32",
                );
                None
            }
        }
    }

    fn u8(&self, value: Option<&Value>, path: DiagnosticPath) -> Option<u8> {
        let value = self.u64(value, path.clone())?;
        match u8::try_from(value) {
            Ok(value) => Some(value),
            Err(_) => {
                self.error(
                    DiagnosticCode::NumericOutOfSchemaRange,
                    path,
                    "integer must fit u8",
                );
                None
            }
        }
    }

    fn f32(&self, value: Option<&Value>, path: DiagnosticPath) -> Option<f32> {
        let value = match value {
            Some(Value::Float(value)) => *value,
            Some(Value::Integer(value)) => *value as f64,
            _ => {
                self.error(
                    DiagnosticCode::WrongType,
                    path,
                    "expected TOML integer or float",
                );
                return None;
            }
        };
        f32_value(
            value,
            path,
            Some(self.span),
            &mut self.diagnostics.borrow_mut(),
        )
    }

    fn id(&self, value: Option<&Value>, path: DiagnosticPath) -> Option<StableId> {
        let value = self.string(value, path.clone())?;
        match StableId::parse(&value) {
            Some(value) => Some(value),
            None => {
                self.error(
                    DiagnosticCode::InvalidId,
                    path,
                    "stable IDs must match [a-z][a-z0-9._-]{0,126}",
                );
                None
            }
        }
    }

    fn token(&self, value: Option<&Value>, path: DiagnosticPath) -> Option<String> {
        self.string(value, path)
    }

    fn bounded(
        &self,
        value: Option<&Value>,
        minimum: f32,
        maximum: f32,
        path: DiagnosticPath,
    ) -> Option<f32> {
        let value = self.f32(value, path.clone())?;
        bounded_f32(
            value,
            minimum,
            maximum,
            path,
            Some(self.span),
            &mut self.diagnostics.borrow_mut(),
        )
    }
}

/// Parse strict TOML 1.0 text into the V1 typed model.
pub fn parse_session_toml(source: &str) -> Result<SessionTomlV1, DiagnosticSet> {
    let root = match toml::from_str::<Value>(source) {
        Ok(value) => value,
        Err(error) => {
            return Err(DiagnosticSet::from_vec(vec![Diagnostic::new(
                DiagnosticCode::TomlSyntax,
                DiagnosticPath::root(),
                Some(SourceSpan::whole(source)),
                error.to_string(),
            )]));
        }
    };
    let mut parser = Parser::new(source);
    let root_path = DiagnosticPath::root();
    let model = parse_root(&mut parser, &root, root_path);
    if parser.diagnostics.borrow().is_empty() {
        match model {
            Some(model) => Ok(model),
            None => Err(DiagnosticSet::from_vec(vec![Diagnostic::new(
                DiagnosticCode::WrongType,
                DiagnosticPath::root(),
                Some(SourceSpan::whole(source)),
                "parser could not build a typed model",
            )])),
        }
    } else {
        Err(DiagnosticSet::from_vec(parser.diagnostics.into_inner()))
    }
}

fn parse_root(parser: &mut Parser, value: &Value, path: DiagnosticPath) -> Option<SessionTomlV1> {
    let table = parser.table(Some(value), path.clone())?;
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
    let schema_version = match table.get("schema_version") {
        Some(value) => parser.u32(Some(value), path.key("schema_version")),
        None => {
            parser.error(
                DiagnosticCode::VersionMissing,
                path.key("schema_version"),
                "required schema version is absent",
            );
            None
        }
    };
    if schema_version.is_some_and(|version| version != SESSION_SCHEMA_VERSION_V1) {
        parser.error(
            DiagnosticCode::VersionUnsupported,
            path.key("schema_version"),
            "only version 1 is accepted",
        );
    }
    let session_id = parser.id(
        parser.required(table, "session_id", &path),
        path.key("session_id"),
    );
    let revision = parser.u64(
        parser.required(table, "revision", &path),
        path.key("revision"),
    );
    let sample_rate_hz = parser.u32(
        parser.required(table, "sample_rate_hz", &path),
        path.key("sample_rate_hz"),
    );
    if sample_rate_hz.is_some_and(|rate| {
        !matches!(
            rate,
            44_100 | 48_000 | 88_200 | 96_000 | 176_400 | 192_000 | 352_800 | 384_000
        )
    }) {
        parser.error(
            DiagnosticCode::NumericOutOfSchemaRange,
            path.key("sample_rate_hz"),
            "sample_rate_hz must be one of the eight supported engine rates",
        );
    }
    let quantum_frames = parser.u32(
        parser.required(table, "quantum_frames", &path),
        path.key("quantum_frames"),
    );
    if quantum_frames.is_some_and(|quantum| quantum == 0) {
        parser.error(
            DiagnosticCode::CapacityZero,
            path.key("quantum_frames"),
            "must be nonzero",
        );
    }
    let render_profile = parse_render_profile(
        parser,
        parser.required(table, "render_profile", &path),
        path.key("render_profile"),
    );
    let output_profile = parse_output_profile(
        parser,
        parser.required(table, "output_profile", &path),
        path.key("output_profile"),
    );
    let limits = parse_limits(
        parser,
        parser.required(table, "limits", &path),
        path.key("limits"),
    );
    let sources = parse_list(
        parser,
        parser.required(table, "sources", &path),
        path.key("sources"),
        parse_source,
    );
    let tracks = parse_list(
        parser,
        parser.required(table, "tracks", &path),
        path.key("tracks"),
        parse_track,
    );
    let submixes = parse_list(
        parser,
        parser.required(table, "submixes", &path),
        path.key("submixes"),
        parse_submix,
    );
    let outputs = parse_list(
        parser,
        parser.required(table, "outputs", &path),
        path.key("outputs"),
        parse_output,
    );
    let routes = parse_list(
        parser,
        parser.required(table, "routes", &path),
        path.key("routes"),
        parse_route,
    );
    let automation = parse_list(
        parser,
        parser.required(table, "automation", &path),
        path.key("automation"),
        parse_automation,
    );
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
        ) => Some(SessionTomlV1 {
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
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<RenderProfile> {
    let table = parser.table(value, path.clone())?;
    parser.keys(table, &["id", "mode"], &path);
    let id = parser.id(parser.required(table, "id", &path), path.key("id"));
    let mode = match parser
        .token(parser.required(table, "mode", &path), path.key("mode"))?
        .as_str()
    {
        "single_thread" => Some(RenderMode::SingleThread),
        "dependency_waves" => Some(RenderMode::DependencyWaves),
        _ => {
            parser.error(
                DiagnosticCode::InvalidEnum,
                path.key("mode"),
                "expected single_thread or dependency_waves",
            );
            None
        }
    };
    Some(RenderProfile {
        id: id?,
        mode: mode?,
    })
}

fn parse_output_profile(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<OutputProfile> {
    let table = parser.table(value, path.clone())?;
    parser.keys(table, &["id", "channels", "sample_format"], &path);
    let id = parser.id(parser.required(table, "id", &path), path.key("id"));
    let channels = parser.u8(
        parser.required(table, "channels", &path),
        path.key("channels"),
    );
    if channels.is_some_and(|value| value != 2) {
        parser.error(
            DiagnosticCode::NumericOutOfSchemaRange,
            path.key("channels"),
            "V1 output must contain exactly two dual-mono channels",
        );
    }
    let sample_format = match parser
        .token(
            parser.required(table, "sample_format", &path),
            path.key("sample_format"),
        )?
        .as_str()
    {
        "f32_planar" => Some(SampleFormat::F32Planar),
        _ => {
            parser.error(
                DiagnosticCode::InvalidEnum,
                path.key("sample_format"),
                "expected f32_planar",
            );
            None
        }
    };
    Some(OutputProfile {
        id: id?,
        channels: channels?,
        sample_format: sample_format?,
    })
}

fn parse_limits(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<SessionLimits> {
    let table = parser.table(value, path.clone())?;
    parser.keys(
        table,
        &["pcm_ring_frames", "control_queue_messages", "memory_bytes"],
        &path,
    );
    let pcm_ring_frames = parser.u64(
        parser.required(table, "pcm_ring_frames", &path),
        path.key("pcm_ring_frames"),
    );
    let control_queue_messages = parser.u64(
        parser.required(table, "control_queue_messages", &path),
        path.key("control_queue_messages"),
    );
    let memory_bytes = parser.u64(
        parser.required(table, "memory_bytes", &path),
        path.key("memory_bytes"),
    );
    for (key, value) in [
        ("pcm_ring_frames", pcm_ring_frames),
        ("control_queue_messages", control_queue_messages),
        ("memory_bytes", memory_bytes),
    ] {
        if value.is_some_and(|value| value == 0) {
            parser.error(
                DiagnosticCode::CapacityZero,
                path.key(key),
                "must be nonzero",
            );
        }
    }
    Some(SessionLimits {
        pcm_ring_frames: pcm_ring_frames?,
        control_queue_messages: control_queue_messages?,
        memory_bytes: memory_bytes?,
    })
}

fn parse_list<T>(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
    parse: fn(&mut Parser, Option<&Value>, DiagnosticPath) -> Option<T>,
) -> Option<Vec<T>> {
    let values = parser.array(value, path.clone())?;
    let mut output = Vec::with_capacity(values.len());
    for (index, value) in values.iter().enumerate() {
        if let Some(item) = parse(parser, Some(value), path.index(index)) {
            output.push(item);
        }
    }
    Some(output)
}

fn parse_source(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<Source> {
    let table = parser.table(value, path.clone())?;
    parser.keys(
        table,
        &["id", "sample_rate_hz", "content", "mapping"],
        &path,
    );
    let id = parser.id(parser.required(table, "id", &path), path.key("id"));
    let sample_rate_hz = parser.u32(
        parser.required(table, "sample_rate_hz", &path),
        path.key("sample_rate_hz"),
    );
    if sample_rate_hz.is_some_and(|rate| rate == 0) {
        parser.error(
            DiagnosticCode::NumericOutOfSchemaRange,
            path.key("sample_rate_hz"),
            "declared source sample rate must be nonzero",
        );
    }
    let content = parse_source_content(
        parser,
        parser.required(table, "content", &path),
        path.key("content"),
    );
    let mapping = parse_source_mapping(
        parser,
        parser.required(table, "mapping", &path),
        path.key("mapping"),
    );
    Some(Source {
        id: id?,
        sample_rate_hz: sample_rate_hz?,
        content: content?,
        mapping: mapping?,
    })
}

fn parse_source_content(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<SourceContent> {
    let table = parser.table(value, path.clone())?;
    parser.keys(table, &["identity", "locator"], &path);
    let identity = parser.string(
        parser.required(table, "identity", &path),
        path.key("identity"),
    );
    let locator = parser.string(
        parser.required(table, "locator", &path),
        path.key("locator"),
    );
    if identity.as_ref().is_some_and(String::is_empty) {
        parser.error(
            DiagnosticCode::NumericOutOfSchemaRange,
            path.key("identity"),
            "must be nonempty",
        );
    }
    if locator.as_ref().is_some_and(String::is_empty) {
        parser.error(
            DiagnosticCode::NumericOutOfSchemaRange,
            path.key("locator"),
            "must be nonempty",
        );
    }
    Some(SourceContent {
        identity: identity?,
        locator: locator?,
    })
}

fn parse_source_mapping(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<SourceMapping> {
    let table = parser.table(value, path.clone())?;
    parser.keys(table, &["channel_count", "region"], &path);
    let channel_count = parser.u8(
        parser.required(table, "channel_count", &path),
        path.key("channel_count"),
    );
    if channel_count.is_some_and(|value| value == 0) {
        parser.error(
            DiagnosticCode::CapacityZero,
            path.key("channel_count"),
            "must be nonzero",
        );
    }
    let region = parse_region(
        parser,
        parser.required(table, "region", &path),
        path.key("region"),
    );
    Some(SourceMapping {
        channel_count: channel_count?,
        region: region?,
    })
}

fn parse_region(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<SourceRegion> {
    let table = parser.table(value, path.clone())?;
    parser.keys(table, &["start_sample", "length_samples"], &path);
    let start_sample = parser.u64(
        parser.required(table, "start_sample", &path),
        path.key("start_sample"),
    );
    let length_samples = parser.u64(
        parser.required(table, "length_samples", &path),
        path.key("length_samples"),
    );
    if length_samples.is_some_and(|value| value == 0) {
        parser.error(
            DiagnosticCode::CapacityZero,
            path.key("length_samples"),
            "must be nonzero",
        );
    }
    if let (Some(start), Some(length)) = (start_sample, length_samples)
        && start.checked_add(length).is_none()
    {
        parser.error(
            DiagnosticCode::SourceRegionOverflow,
            path,
            "source region endpoint overflows u64",
        );
    }
    Some(SourceRegion {
        start_sample: start_sample?,
        length_samples: length_samples?,
    })
}

fn parse_track(parser: &mut Parser, value: Option<&Value>, path: DiagnosticPath) -> Option<Track> {
    let table = parser.table(value, path.clone())?;
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
    let id = parser.id(parser.required(table, "id", &path), path.key("id"));
    let source_id = parser.id(
        parser.required(table, "source_id", &path),
        path.key("source_id"),
    );
    let left_source_channel = parser.u8(
        parser.required(table, "left_source_channel", &path),
        path.key("left_source_channel"),
    );
    let right_source_channel = parser.u8(
        parser.required(table, "right_source_channel", &path),
        path.key("right_source_channel"),
    );
    let builtins = parse_builtins(
        parser,
        parser.required(table, "builtins", &path),
        path.key("builtins"),
    );
    let simd1 = parse_rack(
        parser,
        parser.required(table, "simd1", &path),
        path.key("simd1"),
    );
    let dynamic = parse_rack(
        parser,
        parser.required(table, "dynamic", &path),
        path.key("dynamic"),
    );
    let simd2 = parse_rack(
        parser,
        parser.required(table, "simd2", &path),
        path.key("simd2"),
    );
    let fader = parse_fader(
        parser,
        parser.required(table, "fader", &path),
        path.key("fader"),
    );
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
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<DualMonoBuiltins> {
    let table = parser.table(value, path.clone())?;
    parser.keys(table, &["left", "right"], &path);
    let left = parse_channel_builtins(
        parser,
        parser.required(table, "left", &path),
        path.key("left"),
    );
    let right = parse_channel_builtins(
        parser,
        parser.required(table, "right", &path),
        path.key("right"),
    );
    Some(DualMonoBuiltins {
        left: left?,
        right: right?,
    })
}

fn parse_channel_builtins(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<ChannelBuiltins> {
    let table = parser.table(value, path.clone())?;
    parser.keys(
        table,
        &["polarity_invert", "trim_db", "hpf_hz", "lpf_hz"],
        &path,
    );
    let polarity_invert = parser.bool(
        parser.required(table, "polarity_invert", &path),
        path.key("polarity_invert"),
    );
    let trim_db = parser.f32(
        parser.required(table, "trim_db", &path),
        path.key("trim_db"),
    );
    let hpf_hz = parser.f32(parser.required(table, "hpf_hz", &path), path.key("hpf_hz"));
    let lpf_hz = parser.f32(parser.required(table, "lpf_hz", &path), path.key("lpf_hz"));
    for (field, value) in [("hpf_hz", hpf_hz), ("lpf_hz", lpf_hz)] {
        if value.is_some_and(|value| value < 0.0) {
            parser.error(
                DiagnosticCode::NumericOutOfSchemaRange,
                path.key(field),
                "frequency must be non-negative",
            );
        }
    }
    Some(ChannelBuiltins {
        polarity_invert: polarity_invert?,
        trim_db: trim_db?,
        hpf_hz: hpf_hz?,
        lpf_hz: lpf_hz?,
    })
}

fn parse_rack(parser: &mut Parser, value: Option<&Value>, path: DiagnosticPath) -> Option<Rack> {
    let table = parser.table(value, path.clone())?;
    parser.keys(table, &["effects"], &path);
    let effects = parse_list(
        parser,
        parser.required(table, "effects", &path),
        path.key("effects"),
        parse_effect,
    );
    Some(Rack { effects: effects? })
}

fn parse_effect(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<Effect> {
    let table = parser.table(value, path.clone())?;
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
    let id = parser.id(parser.required(table, "id", &path), path.key("id"));
    let identity = parse_effect_identity(
        parser,
        parser.required(table, "identity", &path),
        path.key("identity"),
    );
    let quality = parse_quality(
        parser,
        parser.required(table, "quality", &path),
        path.key("quality"),
    );
    let bypass = parser.bool(parser.required(table, "bypass", &path), path.key("bypass"));
    let link_mode = parse_link_mode(
        parser,
        parser.required(table, "link_mode", &path),
        path.key("link_mode"),
    );
    let params = parse_list(
        parser,
        parser.required(table, "params", &path),
        path.key("params"),
        parse_param,
    );
    let sidechain = parse_sidechain(
        parser,
        parser.required(table, "sidechain", &path),
        path.key("sidechain"),
    );
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
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<EffectIdentity> {
    let table = parser.table(value, path.clone())?;
    parser.keys(table, &["kind", "effect_id", "cid"], &path);
    match parser
        .token(parser.required(table, "kind", &path), path.key("kind"))?
        .as_str()
    {
        "native" => {
            if table.contains_key("cid") {
                parser.error(
                    DiagnosticCode::UnknownField,
                    path.key("cid"),
                    "native effect identity cannot contain cid",
                );
            }
            let effect_id = parser.id(
                parser.required(table, "effect_id", &path),
                path.key("effect_id"),
            );
            Some(EffectIdentity::Native {
                effect_id: effect_id?,
            })
        }
        "cid" => {
            if table.contains_key("effect_id") {
                parser.error(
                    DiagnosticCode::UnknownField,
                    path.key("effect_id"),
                    "CID effect identity cannot contain effect_id",
                );
            }
            let cid = parser.string(parser.required(table, "cid", &path), path.key("cid"));
            if cid.as_ref().is_some_and(String::is_empty) {
                parser.error(
                    DiagnosticCode::NumericOutOfSchemaRange,
                    path.key("cid"),
                    "must be nonempty",
                );
            }
            Some(EffectIdentity::ThirdPartyCid { cid: cid? })
        }
        _ => {
            parser.error(
                DiagnosticCode::InvalidEnum,
                path.key("kind"),
                "expected native or cid",
            );
            None
        }
    }
}

fn parse_quality(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<EffectQuality> {
    match parser.token(value, path.clone())?.as_str() {
        "draft" => Some(EffectQuality::Draft),
        "normal" => Some(EffectQuality::Normal),
        "high" => Some(EffectQuality::High),
        _ => {
            parser.error(
                DiagnosticCode::InvalidEnum,
                path,
                "expected draft, normal, or high",
            );
            None
        }
    }
}

fn parse_link_mode(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<LinkMode> {
    match parser.token(value, path.clone())?.as_str() {
        "dual_mono" => Some(LinkMode::DualMono),
        "maximum" => Some(LinkMode::Maximum),
        "average" => Some(LinkMode::Average),
        _ => {
            parser.error(
                DiagnosticCode::InvalidEnum,
                path,
                "expected dual_mono, maximum, or average",
            );
            None
        }
    }
}

fn parse_param(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<EffectParam> {
    let table = parser.table(value, path.clone())?;
    parser.keys(table, &["parameter_id", "channel", "unit", "value"], &path);
    let parameter_id = parser.u32(
        parser.required(table, "parameter_id", &path),
        path.key("parameter_id"),
    );
    let channel = parse_parameter_channel(
        parser,
        parser.required(table, "channel", &path),
        path.key("channel"),
    );
    let unit = parse_unit(
        parser,
        parser.required(table, "unit", &path),
        path.key("unit"),
    );
    let value = parser.f32(parser.required(table, "value", &path), path.key("value"));
    let value = validate_parameter_value(parser, value, unit, path.key("value"));
    Some(EffectParam {
        parameter_id: parameter_id?,
        channel: channel?,
        unit: unit?,
        value: value?,
    })
}

fn parse_parameter_channel(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<ParameterChannel> {
    match parser.token(value, path.clone())?.as_str() {
        "left" => Some(ParameterChannel::Left),
        "right" => Some(ParameterChannel::Right),
        "both" => Some(ParameterChannel::Both),
        _ => {
            parser.error(
                DiagnosticCode::InvalidEnum,
                path,
                "expected left, right, or both",
            );
            None
        }
    }
}

fn parse_unit(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<ParameterUnit> {
    match parser.token(value, path.clone())?.as_str() {
        "db" => Some(ParameterUnit::Db),
        "hz" => Some(ParameterUnit::Hz),
        "milliseconds" => Some(ParameterUnit::Milliseconds),
        "samples" => Some(ParameterUnit::Samples),
        "linear" => Some(ParameterUnit::Linear),
        "ratio" => Some(ParameterUnit::Ratio),
        _ => {
            parser.error(
                DiagnosticCode::UnitInvalid,
                path,
                "expected db, hz, milliseconds, samples, linear, or ratio",
            );
            None
        }
    }
}

fn validate_parameter_value(
    parser: &mut Parser,
    value: Option<f32>,
    unit: Option<ParameterUnit>,
    path: DiagnosticPath,
) -> Option<f32> {
    let value = value?;
    let unit = unit?;
    if matches!(
        unit,
        ParameterUnit::Hz
            | ParameterUnit::Milliseconds
            | ParameterUnit::Samples
            | ParameterUnit::Ratio
    ) && value < 0.0
    {
        parser.error(
            DiagnosticCode::NumericOutOfSchemaRange,
            path.clone(),
            "unit requires a non-negative value",
        );
        return None;
    }
    if unit == ParameterUnit::Samples && value.fract() != 0.0 {
        parser.error(
            DiagnosticCode::NumericOutOfSchemaRange,
            path,
            "samples must be integral",
        );
        None
    } else {
        Some(value)
    }
}

fn parse_sidechain(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<SidechainDeclaration> {
    let table = parser.table(value, path.clone())?;
    parser.keys(table, &["kind", "source", "port_id"], &path);
    match parser
        .token(parser.required(table, "kind", &path), path.key("kind"))?
        .as_str()
    {
        "none" => {
            for key in ["source", "port_id"] {
                if table.contains_key(key) {
                    parser.error(
                        DiagnosticCode::UnknownField,
                        path.key(key),
                        "none sidechain cannot contain routing fields",
                    );
                }
            }
            Some(SidechainDeclaration::None)
        }
        "routed" => Some(SidechainDeclaration::Routed(Sidechain {
            source: parse_route_source(
                parser,
                parser.required(table, "source", &path),
                path.key("source"),
            )?,
            port_id: parser.id(
                parser.required(table, "port_id", &path),
                path.key("port_id"),
            )?,
        })),
        _ => {
            parser.error(
                DiagnosticCode::InvalidEnum,
                path.key("kind"),
                "expected none or routed",
            );
            None
        }
    }
}

fn parse_fader(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<DualMonoFader> {
    let table = parser.table(value, path.clone())?;
    parser.keys(
        table,
        &["left_db", "right_db", "left_mute", "right_mute"],
        &path,
    );
    let left_db = parser.f32(
        parser.required(table, "left_db", &path),
        path.key("left_db"),
    );
    let right_db = parser.f32(
        parser.required(table, "right_db", &path),
        path.key("right_db"),
    );
    let left_mute = parser.bool(
        parser.required(table, "left_mute", &path),
        path.key("left_mute"),
    );
    let right_mute = parser.bool(
        parser.required(table, "right_mute", &path),
        path.key("right_mute"),
    );
    Some(DualMonoFader {
        left_db: left_db?,
        right_db: right_db?,
        left_mute: left_mute?,
        right_mute: right_mute?,
    })
}

fn parse_matrix_or_pan(
    parser: &mut Parser,
    table: &Table,
    path: &DiagnosticPath,
) -> Option<MatrixOrPan> {
    match (
        parser.optional(table, "pan"),
        parser.optional(table, "matrix"),
    ) {
        (Some(_), Some(_)) => {
            parser.error(
                DiagnosticCode::WrongType,
                path.clone(),
                "exactly one of pan or matrix is required",
            );
            None
        }
        (None, None) => {
            parser.error(
                DiagnosticCode::MissingField,
                path.clone(),
                "exactly one of pan or matrix is required",
            );
            None
        }
        (Some(value), None) => {
            let pan_path = path.key("pan");
            let pan = parser.table(Some(value), pan_path.clone())?;
            parser.keys(pan, &["left", "right", "smoothing_samples"], &pan_path);
            let left = parser.bounded(
                parser.required(pan, "left", &pan_path),
                -1.0,
                1.0,
                pan_path.key("left"),
            );
            let right = parser.bounded(
                parser.required(pan, "right", &pan_path),
                -1.0,
                1.0,
                pan_path.key("right"),
            );
            let smoothing_samples = parser.u32(
                parser.required(pan, "smoothing_samples", &pan_path),
                pan_path.key("smoothing_samples"),
            );
            Some(MatrixOrPan::Pan {
                left: left?,
                right: right?,
                smoothing_samples: smoothing_samples?,
            })
        }
        (None, Some(value)) => {
            let matrix_path = path.key("matrix");
            let matrix = parser.table(Some(value), matrix_path.clone())?;
            parser.keys(
                matrix,
                &["ll", "lr", "rl", "rr", "smoothing_samples"],
                &matrix_path,
            );
            let ll = parser.f32(
                parser.required(matrix, "ll", &matrix_path),
                matrix_path.key("ll"),
            );
            let lr = parser.f32(
                parser.required(matrix, "lr", &matrix_path),
                matrix_path.key("lr"),
            );
            let rl = parser.f32(
                parser.required(matrix, "rl", &matrix_path),
                matrix_path.key("rl"),
            );
            let rr = parser.f32(
                parser.required(matrix, "rr", &matrix_path),
                matrix_path.key("rr"),
            );
            let smoothing_samples = parser.u32(
                parser.required(matrix, "smoothing_samples", &matrix_path),
                matrix_path.key("smoothing_samples"),
            );
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
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<Submix> {
    let table = parser.table(value, path.clone())?;
    parser.keys(table, &["id"], &path);
    Some(Submix {
        id: parser.id(parser.required(table, "id", &path), path.key("id"))?,
    })
}

fn parse_output(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<Output> {
    let table = parser.table(value, path.clone())?;
    parser.keys(table, &["id"], &path);
    Some(Output {
        id: parser.id(parser.required(table, "id", &path), path.key("id"))?,
    })
}

fn parse_route(parser: &mut Parser, value: Option<&Value>, path: DiagnosticPath) -> Option<Route> {
    let table = parser.table(value, path.clone())?;
    parser.keys(
        table,
        &["id", "source", "destination", "channel_matrix", "gain_db"],
        &path,
    );
    let id = parser.id(parser.required(table, "id", &path), path.key("id"));
    let source = parse_route_source(
        parser,
        parser.required(table, "source", &path),
        path.key("source"),
    );
    let destination = parse_route_destination(
        parser,
        parser.required(table, "destination", &path),
        path.key("destination"),
    );
    let channel_matrix = parse_channel_matrix(
        parser,
        parser.required(table, "channel_matrix", &path),
        path.key("channel_matrix"),
    );
    let gain_db = parser.f32(
        parser.required(table, "gain_db", &path),
        path.key("gain_db"),
    );
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
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<RouteSource> {
    let table = parser.table(value, path.clone())?;
    parser.keys(table, &["kind", "track_id", "tap", "submix_id"], &path);
    match parser
        .token(parser.required(table, "kind", &path), path.key("kind"))?
        .as_str()
    {
        "track" => {
            if table.contains_key("submix_id") {
                parser.error(
                    DiagnosticCode::UnknownField,
                    path.key("submix_id"),
                    "track source cannot contain submix_id",
                );
            }
            Some(RouteSource::Track {
                track_id: parser.id(
                    parser.required(table, "track_id", &path),
                    path.key("track_id"),
                )?,
                tap: parse_tap(
                    parser,
                    parser.required(table, "tap", &path),
                    path.key("tap"),
                )?,
            })
        }
        "submix_output" => {
            for key in ["track_id", "tap"] {
                if table.contains_key(key) {
                    parser.error(
                        DiagnosticCode::UnknownField,
                        path.key(key),
                        "submix_output source cannot contain track fields",
                    );
                }
            }
            Some(RouteSource::SubmixOutput {
                submix_id: parser.id(
                    parser.required(table, "submix_id", &path),
                    path.key("submix_id"),
                )?,
            })
        }
        _ => {
            parser.error(
                DiagnosticCode::InvalidEnum,
                path.key("kind"),
                "expected track or submix_output",
            );
            None
        }
    }
}

fn parse_route_destination(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<RouteDestination> {
    let table = parser.table(value, path.clone())?;
    parser.keys(table, &["kind", "submix_id", "output_id"], &path);
    match parser
        .token(parser.required(table, "kind", &path), path.key("kind"))?
        .as_str()
    {
        "submix_input" => {
            if table.contains_key("output_id") {
                parser.error(
                    DiagnosticCode::UnknownField,
                    path.key("output_id"),
                    "submix_input cannot contain output_id",
                );
            }
            Some(RouteDestination::SubmixInput {
                submix_id: parser.id(
                    parser.required(table, "submix_id", &path),
                    path.key("submix_id"),
                )?,
            })
        }
        "output_input" => {
            if table.contains_key("submix_id") {
                parser.error(
                    DiagnosticCode::UnknownField,
                    path.key("submix_id"),
                    "output_input cannot contain submix_id",
                );
            }
            Some(RouteDestination::OutputInput {
                output_id: parser.id(
                    parser.required(table, "output_id", &path),
                    path.key("output_id"),
                )?,
            })
        }
        _ => {
            parser.error(
                DiagnosticCode::InvalidEnum,
                path.key("kind"),
                "expected submix_input or output_input",
            );
            None
        }
    }
}

fn parse_channel_matrix(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<ChannelMatrix> {
    let table = parser.table(value, path.clone())?;
    parser.keys(table, &["ll", "lr", "rl", "rr"], &path);
    let ll = parser.f32(parser.required(table, "ll", &path), path.key("ll"));
    let lr = parser.f32(parser.required(table, "lr", &path), path.key("lr"));
    let rl = parser.f32(parser.required(table, "rl", &path), path.key("rl"));
    let rr = parser.f32(parser.required(table, "rr", &path), path.key("rr"));
    Some(ChannelMatrix {
        ll: ll?,
        lr: lr?,
        rl: rl?,
        rr: rr?,
    })
}

fn parse_tap(parser: &mut Parser, value: Option<&Value>, path: DiagnosticPath) -> Option<SendTap> {
    match parser.token(value, path.clone())?.as_str() {
        "input" => Some(SendTap::Input),
        "post_input_builtins" => Some(SendTap::PostInputBuiltins),
        "post_simd1" => Some(SendTap::PostSimd1),
        "post_dynamic" => Some(SendTap::PostDynamic),
        "post_simd2_pre_fader" => Some(SendTap::PostSimd2PreFader),
        "post_fader" => Some(SendTap::PostFader),
        "post_matrix" => Some(SendTap::PostMatrix),
        _ => {
            parser.error(DiagnosticCode::InvalidEnum, path, "invalid send tap");
            None
        }
    }
}

fn parse_automation(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<Automation> {
    let table = parser.table(value, path.clone())?;
    parser.keys(table, &["id", "target", "segments"], &path);
    let id = parser.id(parser.required(table, "id", &path), path.key("id"));
    let target = parse_target(
        parser,
        parser.required(table, "target", &path),
        path.key("target"),
    );
    let segments = parse_list(
        parser,
        parser.required(table, "segments", &path),
        path.key("segments"),
        parse_segment,
    );
    Some(Automation {
        id: id?,
        target: target?,
        segments: segments?,
    })
}

fn parse_target(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<AutomationTarget> {
    let table = parser.table(value, path.clone())?;
    parser.keys(
        table,
        &["entity_id", "rack", "effect_id", "parameter_id", "channel"],
        &path,
    );
    let entity_id = parser.id(
        parser.required(table, "entity_id", &path),
        path.key("entity_id"),
    );
    let rack = parse_rack_name(
        parser,
        parser.required(table, "rack", &path),
        path.key("rack"),
    );
    let effect_id = parser.id(
        parser.required(table, "effect_id", &path),
        path.key("effect_id"),
    );
    let parameter_id = parser.u32(
        parser.required(table, "parameter_id", &path),
        path.key("parameter_id"),
    );
    let channel = parse_parameter_channel(
        parser,
        parser.required(table, "channel", &path),
        path.key("channel"),
    );
    Some(AutomationTarget {
        entity_id: entity_id?,
        rack: rack?,
        effect_id: effect_id?,
        parameter_id: parameter_id?,
        channel: channel?,
    })
}

fn parse_rack_name(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<RackName> {
    match parser.token(value, path.clone())?.as_str() {
        "simd1" => Some(RackName::Simd1),
        "dynamic" => Some(RackName::Dynamic),
        "simd2" => Some(RackName::Simd2),
        _ => {
            parser.error(
                DiagnosticCode::InvalidEnum,
                path,
                "expected simd1, dynamic, or simd2",
            );
            None
        }
    }
}

fn parse_segment(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<AutomationSegment> {
    let table = parser.table(value, path.clone())?;
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
    let shape = parse_shape(
        parser,
        parser.required(table, "shape", &path),
        path.key("shape"),
    );
    let start_sample = parser.u64(
        parser.required(table, "start_sample", &path),
        path.key("start_sample"),
    );
    let end_sample = parser.u64(
        parser.required(table, "end_sample", &path),
        path.key("end_sample"),
    );
    let start_value = parser.f32(
        parser.required(table, "start_value", &path),
        path.key("start_value"),
    );
    let end_value = parser.f32(
        parser.required(table, "end_value", &path),
        path.key("end_value"),
    );
    let unit = parse_unit(
        parser,
        parser.required(table, "unit", &path),
        path.key("unit"),
    );
    let start_value = validate_parameter_value(parser, start_value, unit, path.key("start_value"));
    let end_value = validate_parameter_value(parser, end_value, unit, path.key("end_value"));
    if let (Some(start), Some(end)) = (start_sample, end_sample)
        && start >= end
    {
        parser.error(
            DiagnosticCode::AutomationInvalidRange,
            path.key("end_sample"),
            "start_sample must precede end_sample",
        );
    }
    if shape == Some(AutomationShape::Exponential)
        && (start_value.is_some_and(|value| value <= 0.0)
            || end_value.is_some_and(|value| value <= 0.0))
    {
        parser.error(
            DiagnosticCode::AutomationInvalidRange,
            path.clone(),
            "exponential values must be positive",
        );
    }
    Some(AutomationSegment {
        shape: shape?,
        start_sample: start_sample?,
        end_sample: end_sample?,
        start_value: start_value?,
        end_value: end_value?,
        unit: unit?,
    })
}

fn parse_shape(
    parser: &mut Parser,
    value: Option<&Value>,
    path: DiagnosticPath,
) -> Option<AutomationShape> {
    match parser.token(value, path.clone())?.as_str() {
        "step" => Some(AutomationShape::Step),
        "linear" => Some(AutomationShape::Linear),
        "exponential" => Some(AutomationShape::Exponential),
        _ => {
            parser.error(
                DiagnosticCode::InvalidEnum,
                path,
                "expected step, linear, or exponential",
            );
            None
        }
    }
}
