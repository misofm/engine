//! Semantic validation owned by issue 004, deliberately before graph/DSP/effect resolution.
use crate::{
    AutomationShape, AutomationTarget, Diagnostic, DiagnosticCode, DiagnosticSet, Effect,
    MatrixOrPan, ParameterChannel, ParameterUnit, Rack, RackName, RenderMode, RouteDestination,
    RouteSource, SESSION_SCHEMA_VERSION_V1, SessionToml, Source, Track,
    diagnostic::{MAXIMUM_SESSION_DIAGNOSTICS, PathRef},
};
use miso_engine_core::{SampleRateHz, is_launch_sample_rate};
use std::collections::{HashMap, HashSet};
#[derive(Clone, Copy)]
enum GraphEntity<'a> {
    Track(&'a Track),
    Submix,
    Output,
}
struct Index<'a> {
    sources: HashMap<&'a str, &'a Source>,
    graph: HashMap<&'a str, GraphEntity<'a>>,
}
#[derive(Default)]
struct LocalUniqueness<'a> {
    effect_ids: HashSet<&'a str>,
    parameters: HashSet<(u32, u8)>,
}
pub(crate) fn validate_session(session: &SessionToml) -> Result<(), DiagnosticSet> {
    let mut diagnostics = Vec::new();
    let root = PathRef::ROOT;
    if session.schema_version != SESSION_SCHEMA_VERSION_V1 {
        error(
            &mut diagnostics,
            DiagnosticCode::VersionUnsupported,
            &root.key("schema_version"),
            "only version 1 is accepted",
        );
    }
    if !is_launch_sample_rate(SampleRateHz(session.sample_rate_hz)) {
        error(
            &mut diagnostics,
            DiagnosticCode::SampleRateUnsupportedAtLaunch,
            &root.key("sample_rate_hz"),
            "launch sample_rate_hz must be one of 44100, 48000, 88200, or 96000 Hz",
        );
    }
    if session.render_profile.mode != RenderMode::SingleThread {
        error(
            &mut diagnostics,
            DiagnosticCode::RenderModeUnsupportedAtLaunch,
            &root.key("render_profile").key("mode"),
            "launch render_profile.mode must be single_thread",
        );
    }
    if session.quantum_frames == 0 {
        error(
            &mut diagnostics,
            DiagnosticCode::CapacityZero,
            &root.key("quantum_frames"),
            "quantum_frames must be nonzero",
        );
    }
    if session.output_profile.channels != 2 {
        error(
            &mut diagnostics,
            DiagnosticCode::NumericOutOfSchemaRange,
            &root.key("output_profile").key("channels"),
            "V1 output must contain exactly two dual-mono channels",
        );
    }

    validate_u64(&mut diagnostics, session.revision, &root.key("revision"));
    // One pass establishes uniqueness and builds every global cross-reference index.
    let sources_path = root.key("sources");
    let mut sources = HashMap::with_capacity(session.sources.len());
    for (position, source) in session.sources.iter().enumerate() {
        if sources.insert(source.id.as_str(), source).is_some() {
            duplicate(&mut diagnostics, &sources_path.index(position).key("id"));
        }
    }

    let graph_capacity = session
        .tracks
        .len()
        .saturating_add(session.submixes.len())
        .saturating_add(session.outputs.len());
    let mut graph = HashMap::with_capacity(graph_capacity);
    let tracks_path = root.key("tracks");
    for (position, track) in session.tracks.iter().enumerate() {
        if graph
            .insert(track.id.as_str(), GraphEntity::Track(track))
            .is_some()
        {
            duplicate(&mut diagnostics, &tracks_path.index(position).key("id"));
        }
    }
    let submixes_path = root.key("submixes");
    for (position, submix) in session.submixes.iter().enumerate() {
        if graph
            .insert(submix.id.as_str(), GraphEntity::Submix)
            .is_some()
        {
            duplicate(&mut diagnostics, &submixes_path.index(position).key("id"));
        }
    }
    let outputs_path = root.key("outputs");
    for (position, output) in session.outputs.iter().enumerate() {
        if graph
            .insert(output.id.as_str(), GraphEntity::Output)
            .is_some()
        {
            duplicate(&mut diagnostics, &outputs_path.index(position).key("id"));
        }
    }

    let routes_path = root.key("routes");
    let mut route_ids = HashSet::with_capacity(session.routes.len());
    for (position, route) in session.routes.iter().enumerate() {
        if !route_ids.insert(route.id.as_str()) {
            error(
                &mut diagnostics,
                DiagnosticCode::DuplicateId,
                &routes_path.index(position).key("id"),
                "route ID is repeated",
            );
        }
    }
    let automations_path = root.key("automation");
    let mut automation_ids = HashSet::with_capacity(session.automation.len());
    for (position, automation) in session.automation.iter().enumerate() {
        if !automation_ids.insert(automation.id.as_str()) {
            error(
                &mut diagnostics,
                DiagnosticCode::DuplicateId,
                &automations_path.index(position).key("id"),
                "automation ID is repeated",
            );
        }
    }

    let index = Index { sources, graph };
    let mut local = LocalUniqueness::default();
    validate_sources(session, &root, &mut diagnostics);
    validate_tracks(session, &index, &root, &mut diagnostics, &mut local);
    validate_routes(session, &index, &root, &mut diagnostics);
    validate_automation(session, &index, &root, &mut diagnostics);

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(DiagnosticSet::from_vec(diagnostics))
    }
}

fn duplicate(diagnostics: &mut Vec<Diagnostic>, path: &PathRef<'_>) {
    error(
        diagnostics,
        DiagnosticCode::DuplicateId,
        path,
        "entity ID is repeated",
    );
}

fn validate_sources(session: &SessionToml, root: &PathRef<'_>, diagnostics: &mut Vec<Diagnostic>) {
    let sources_path = root.key("sources");
    for (position, source) in session.sources.iter().enumerate() {
        let path = sources_path.index(position);
        if !valid_source_content_identity(&source.content) {
            error(
                diagnostics,
                DiagnosticCode::SourceContentIdentityFormat,
                &path.key("content"),
                "source content must match sha256:[0-9a-f]{64}",
            );
        }
        if source.channels == 0 {
            error(
                diagnostics,
                DiagnosticCode::CapacityZero,
                &path.key("channels"),
                "source channels must be nonzero",
            );
        }
        if source.frames == 0 {
            error(
                diagnostics,
                DiagnosticCode::CapacityZero,
                &path.key("frames"),
                "source frames must be nonzero",
            );
        }
        validate_u64(diagnostics, source.frames, &path.key("frames"));
    }
}

fn valid_source_content_identity(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(|digest| {
        digest.len() == 64
            && digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}

fn validate_tracks<'a>(
    session: &'a SessionToml,
    index: &Index<'_>,
    root: &PathRef<'_>,
    diagnostics: &mut Vec<Diagnostic>,
    local: &mut LocalUniqueness<'a>,
) {
    let tracks_path = root.key("tracks");
    for (position, track) in session.tracks.iter().enumerate() {
        let path = tracks_path.index(position);
        let source = index.sources.get(track.source_id.as_str()).copied();
        if source.is_none() {
            error(
                diagnostics,
                DiagnosticCode::MissingEntityReference,
                &path.key("source_id"),
                "track source_id is not a declared source",
            );
        }
        if let Some(source) = source {
            for (field, channel) in [
                ("left_source_channel", track.left_source_channel),
                ("right_source_channel", track.right_source_channel),
            ] {
                if channel >= source.channels {
                    error(
                        diagnostics,
                        DiagnosticCode::SourceChannelIndexOutOfRange,
                        &path.key(field),
                        "track source channel index exceeds declared source channels",
                    );
                }
            }
        }

        for (channel, values) in [
            ("left", &track.builtins.left),
            ("right", &track.builtins.right),
        ] {
            let builtins_path = path.key("builtins");
            let channel_path = builtins_path.key(channel);
            validate_finite(diagnostics, values.trim_db, &channel_path.key("trim_db"));
            validate_nonnegative_finite(diagnostics, values.hpf_hz, &channel_path.key("hpf_hz"));
            validate_nonnegative_finite(diagnostics, values.lpf_hz, &channel_path.key("lpf_hz"));
            // A flat integer domain, checked here alongside the finite checks. The upper bound is
            // the schema's, not the DSP's: it is what bounds the ring allocation a hostile
            // session can demand, which is why it is stage-2 schema work rather than issue-007
            // Nyquist work.
            if values.delay_samples > crate::CHANNEL_BUILTIN_DELAY_SAMPLES_MAXIMUM {
                error(
                    diagnostics,
                    DiagnosticCode::NumericOutOfSchemaRange,
                    &channel_path.key("delay_samples"),
                    "builtin delay_samples exceeds the schema maximum of 48000",
                );
            }
        }
        let fader_path = path.key("fader");
        validate_finite(diagnostics, track.fader.left_db, &fader_path.key("left_db"));
        validate_finite(
            diagnostics,
            track.fader.right_db,
            &fader_path.key("right_db"),
        );
        match track.matrix_or_pan {
            MatrixOrPan::Pan { left, right, .. } => {
                let pan_path = path.key("pan");
                validate_finite_range(diagnostics, left, -1.0, 1.0, &pan_path.key("left"));
                validate_finite_range(diagnostics, right, -1.0, 1.0, &pan_path.key("right"));
            }
            MatrixOrPan::Matrix { ll, lr, rl, rr, .. } => {
                let matrix_path = path.key("matrix");
                for (field, value) in [("ll", ll), ("lr", lr), ("rl", rl), ("rr", rr)] {
                    validate_finite(diagnostics, value, &matrix_path.key(field));
                }
            }
        }
        for (name, rack) in [
            ("simd1", &track.simd1),
            ("dynamic", &track.dynamic),
            ("simd2", &track.simd2),
        ] {
            validate_rack(diagnostics, index, rack, &path.key(name), local);
        }
    }
}

fn validate_rack<'a>(
    diagnostics: &mut Vec<Diagnostic>,
    index: &Index<'_>,
    rack: &'a Rack,
    path: &PathRef<'_>,
    local: &mut LocalUniqueness<'a>,
) {
    let effects_path = path.key("effects");
    local.effect_ids.clear();
    for (position, effect) in rack.effects.iter().enumerate() {
        let effect_path = effects_path.index(position);
        if !local.effect_ids.insert(effect.id.as_str()) {
            error(
                diagnostics,
                DiagnosticCode::DuplicateId,
                &effect_path.key("id"),
                "effect ID is repeated in a rack",
            );
        }
        validate_effect(diagnostics, index, effect, &effect_path, local);
    }
}

fn validate_effect(
    diagnostics: &mut Vec<Diagnostic>,
    index: &Index<'_>,
    effect: &Effect,
    path: &PathRef<'_>,
    local: &mut LocalUniqueness<'_>,
) {
    if let crate::EffectIdentity::ThirdPartyCid { cid } = &effect.identity
        && cid.is_empty()
    {
        error(
            diagnostics,
            DiagnosticCode::NumericOutOfSchemaRange,
            &path.key("identity").key("cid"),
            "third-party CID must be nonempty",
        );
    }
    if let crate::SidechainDeclaration::Routed(sidechain) = &effect.sidechain {
        let sidechain_path = path.key("sidechain");
        validate_route_source(
            diagnostics,
            index,
            &sidechain.source,
            &sidechain_path.key("source"),
        );
    }
    let params_path = path.key("params");
    local.parameters.clear();
    for (position, parameter) in effect.params.iter().enumerate() {
        let parameter_path = params_path.index(position);
        let channel = match parameter.channel {
            crate::ParameterChannel::Left => 0_u8,
            crate::ParameterChannel::Right => 1,
            crate::ParameterChannel::Both => 2,
        };
        if !local.parameters.insert((parameter.parameter_id, channel)) {
            error(
                diagnostics,
                DiagnosticCode::DuplicateId,
                &parameter_path.key("parameter_id"),
                "parameter ID/channel is repeated",
            );
        }
        validate_unit_value(
            diagnostics,
            parameter.value,
            parameter.unit,
            &parameter_path.key("value"),
        );
    }
}

fn validate_routes(
    session: &SessionToml,
    index: &Index<'_>,
    root: &PathRef<'_>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let routes_path = root.key("routes");
    for (position, route) in session.routes.iter().enumerate() {
        let path = routes_path.index(position);
        validate_route_source(diagnostics, index, &route.source, &path.key("source"));
        validate_route_destination(
            diagnostics,
            index,
            &route.destination,
            &path.key("destination"),
        );
        validate_finite(diagnostics, route.gain_db, &path.key("gain_db"));
        let matrix_path = path.key("channel_matrix");
        for (field, value) in [
            ("ll", route.channel_matrix.ll),
            ("lr", route.channel_matrix.lr),
            ("rl", route.channel_matrix.rl),
            ("rr", route.channel_matrix.rr),
        ] {
            validate_finite(diagnostics, value, &matrix_path.key(field));
        }
    }
}

fn validate_route_source(
    diagnostics: &mut Vec<Diagnostic>,
    index: &Index<'_>,
    source: &RouteSource,
    path: &PathRef<'_>,
) {
    let (valid, leaf) = match source {
        RouteSource::Track { track_id, .. } => (
            matches!(
                index.graph.get(track_id.as_str()),
                Some(GraphEntity::Track(_))
            ),
            "track_id",
        ),
        RouteSource::SubmixOutput { submix_id } => (
            matches!(
                index.graph.get(submix_id.as_str()),
                Some(GraphEntity::Submix)
            ),
            "submix_id",
        ),
    };
    if !valid {
        error(
            diagnostics,
            DiagnosticCode::MissingEntityReference,
            &path.key(leaf),
            "route source entity is not declared with the required role",
        );
    }
}

fn validate_route_destination(
    diagnostics: &mut Vec<Diagnostic>,
    index: &Index<'_>,
    destination: &RouteDestination,
    path: &PathRef<'_>,
) {
    let (valid, leaf) = match destination {
        RouteDestination::SubmixInput { submix_id } => (
            matches!(
                index.graph.get(submix_id.as_str()),
                Some(GraphEntity::Submix)
            ),
            "submix_id",
        ),
        RouteDestination::OutputInput { output_id } => (
            matches!(
                index.graph.get(output_id.as_str()),
                Some(GraphEntity::Output)
            ),
            "output_id",
        ),
    };
    if !valid {
        error(
            diagnostics,
            DiagnosticCode::MissingEntityReference,
            &path.key(leaf),
            "route destination entity is not declared with the required role",
        );
    }
}

/// The one `effect_id` a `rack = "builtins"` automation target may name.
///
/// The strip is a single fixed object, not a rack of instances, so there is nothing to identify --
/// but Session V1 has no optional keys (`docs/SESSION_SCHEMA_V1.md`), so the field cannot simply be
/// omitted. It carries a fixed, validated literal instead, which is the C2 rule applied to a key
/// whose value is determined: the document still declares five target fields and the reader still
/// reads five, and a target that names anything else is refused rather than silently ignored.
pub const BUILTIN_AUTOMATION_EFFECT_ID: &str = "strip";

/// The builtin parameters a session may name as an automation target, `(id, per_lane)`.
///
/// # Why this table is spelled here
///
/// `miso-engine-session` depends on `miso-engine-core` and nothing else -- that is a policy
/// (`scripts/check-session-policy.sh`), not an accident -- so this crate cannot read
/// `BUILTIN_PARAMETER_DESCRIPTORS` and this is a deliberate second spelling of it, exactly as
/// `scripts/check-parameter-metadata-v1.py` is a second spelling of the command-kind list. The two
/// are held together by `miso_engine_builtins_compiler`'s
/// `builtin_automation_targets_match_the_parameter_abi`, which can see both crates and compares
/// them row by row: a descriptor whose `update_rate` moves without this table moving is red there.
///
/// # Why the list is exactly the block-target rows
///
/// A target names something the render plane can be *told* to change. The rows that declare
/// `BuiltinParameterUpdateRate::PreparedOnly` -- `hpf_hz` (3), `lpf_hz` (4) and `delay_samples`
/// (11) -- have no post-preparation write path at all, so an automation span addressed at one of
/// them could only ever be inert syntax. They are refused, and the refusal is the ruling: the
/// deferred filter tier and the delay ruling are reopened by adding a row here, not by writing a
/// session that quietly does nothing.
///
/// `per_lane` is the descriptor's `BuiltinParameterScope`: a `PerLane` parameter may be addressed
/// `left`, `right` or `both`, while the four matrix coefficients are one shared 2x2 and can only
/// be addressed `both`.
pub const BUILTIN_AUTOMATION_TARGETS: [(u32, bool); 8] = [
    // `polarity_invert` and `trim_db`: live since #210 phase 3.
    (1, true),
    (2, true),
    // `fader_db` and `mute`: live since #140 B.
    (5, true),
    (6, true),
    // `matrix_ll/lr/rl/rr`: live since #137 D1, and `MatrixShared`.
    (7, false),
    (8, false),
    (9, false),
    (10, false),
];

/// Validate one `rack = "builtins"` automation target against the builtin parameter ABI.
///
/// # What this does and does not unblock (#178, ruled by #210's D2)
///
/// It extends the target **vocabulary** and nothing else. No lowering reads the session's
/// automation table -- for this rack or for any of the other three -- so a valid `builtins` target
/// is valid-and-inert syntax today: it authors, it round-trips, it survives the canonical writer,
/// and it renders nothing. Builtin automation *rendering* is gated on issue #140's span feed,
/// whose natural destination is the very drains #210 phases 1 and 3 built
/// (`TrackInputRecord`, `TrackFaderRecord`, `TrackControlRecord`), because a span's
/// block-first-sample semantics already match the drain contract. Nothing here builds that feed
/// and nothing here should be read as having built it.
fn validate_builtin_automation_target(
    target: &AutomationTarget,
    path: &PathRef<'_>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let target_path = path.key("target");
    if target.effect_id.as_str() != BUILTIN_AUTOMATION_EFFECT_ID {
        error(
            diagnostics,
            DiagnosticCode::MissingEntityReference,
            &target_path.key("effect_id"),
            "builtins automation must name the strip",
        );
    }
    let Some((_, per_lane)) = BUILTIN_AUTOMATION_TARGETS
        .iter()
        .find(|(id, _)| *id == target.parameter_id)
    else {
        error(
            diagnostics,
            DiagnosticCode::MissingEntityReference,
            &target_path.key("parameter_id"),
            "parameter ID is not an automatable builtin parameter",
        );
        return;
    };
    if !*per_lane && target.channel != ParameterChannel::Both {
        error(
            diagnostics,
            DiagnosticCode::InvalidEnum,
            &target_path.key("channel"),
            "shared builtin parameters are addressed as both",
        );
    }
}

fn validate_automation(
    session: &SessionToml,
    index: &Index<'_>,
    root: &PathRef<'_>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let automations_path = root.key("automation");
    for (position, automation) in session.automation.iter().enumerate() {
        let path = automations_path.index(position);
        if automation.segments.is_empty() {
            error(
                diagnostics,
                DiagnosticCode::NumericOutOfSchemaRange,
                &path.key("segments"),
                "automation must declare at least one segment",
            );
        }

        let track = match index.graph.get(automation.target.entity_id.as_str()) {
            Some(GraphEntity::Track(track)) => Some(*track),
            _ => {
                error(
                    diagnostics,
                    DiagnosticCode::MissingEntityReference,
                    &path.key("target").key("entity_id"),
                    "automation target must be a declared track",
                );
                None
            }
        };
        if let Some(track) = track {
            let rack = match automation.target.rack {
                RackName::Simd1 => Some(&track.simd1),
                RackName::Dynamic => Some(&track.dynamic),
                RackName::Simd2 => Some(&track.simd2),
                // The strip is not a rack of effects, so there is nothing to search; the arm
                // below validates the target against the builtin parameter ABI instead.
                RackName::Builtins => None,
            };
            if let Some(rack) = rack {
                // Rack size is resource-bounded; this is one of two intentional local searches.
                let effect = rack
                    .effects
                    .iter()
                    .find(|effect| effect.id == automation.target.effect_id);
                if let Some(effect) = effect {
                    // Parameter count is resource-bounded; keep the local `(id, channel)` search.
                    if !effect.params.iter().any(|parameter| {
                        parameter.parameter_id == automation.target.parameter_id
                            && parameter.channel == automation.target.channel
                    }) {
                        error(
                            diagnostics,
                            DiagnosticCode::MissingEntityReference,
                            &path.key("target").key("parameter_id"),
                            "parameter/channel is absent from selected effect",
                        );
                    }
                } else {
                    error(
                        diagnostics,
                        DiagnosticCode::MissingEntityReference,
                        &path.key("target").key("effect_id"),
                        "effect ID is absent from selected rack",
                    );
                }
            } else {
                validate_builtin_automation_target(&automation.target, &path, diagnostics);
            }
        }

        let segments_path = path.key("segments");
        let mut previous_start = None;
        let mut previous_end = None;
        for (segment_position, segment) in automation.segments.iter().enumerate() {
            let segment_path = segments_path.index(segment_position);
            validate_u64(
                diagnostics,
                segment.start_sample,
                &segment_path.key("start_sample"),
            );
            validate_u64(
                diagnostics,
                segment.end_sample,
                &segment_path.key("end_sample"),
            );
            if segment.start_sample >= segment.end_sample {
                error(
                    diagnostics,
                    DiagnosticCode::AutomationInvalidRange,
                    &segment_path.key("end_sample"),
                    "end_sample must be greater than start_sample",
                );
            }
            if previous_start.is_some_and(|start| segment.start_sample < start) {
                error(
                    diagnostics,
                    DiagnosticCode::AutomationOutOfOrder,
                    &segment_path.key("start_sample"),
                    "segment starts before its predecessor",
                );
            } else if previous_end.is_some_and(|end| segment.start_sample < end) {
                error(
                    diagnostics,
                    DiagnosticCode::AutomationSegmentOverlap,
                    &segment_path.key("start_sample"),
                    "segment overlaps its predecessor",
                );
            }
            previous_start = Some(segment.start_sample);
            previous_end = Some(segment.end_sample);
            validate_unit_value(
                diagnostics,
                segment.start_value,
                segment.unit,
                &segment_path.key("start_value"),
            );
            validate_unit_value(
                diagnostics,
                segment.end_value,
                segment.unit,
                &segment_path.key("end_value"),
            );
            if segment.shape == AutomationShape::Exponential {
                for (field, value) in [
                    ("start_value", segment.start_value),
                    ("end_value", segment.end_value),
                ] {
                    if value.is_finite() && value <= 0.0 {
                        error(
                            diagnostics,
                            DiagnosticCode::AutomationInvalidRange,
                            &segment_path.key(field),
                            "exponential values must be positive",
                        );
                    }
                }
            }
        }
    }
}

fn validate_u64(diagnostics: &mut Vec<Diagnostic>, value: u64, path: &PathRef<'_>) {
    if value > i64::MAX as u64 {
        error(
            diagnostics,
            DiagnosticCode::NumericOutOfSchemaRange,
            path,
            "integer exceeds the TOML i64 range",
        );
    }
}

fn validate_unit_value(
    diagnostics: &mut Vec<Diagnostic>,
    value: f32,
    unit: ParameterUnit,
    path: &PathRef<'_>,
) {
    validate_finite(diagnostics, value, path);
    if matches!(
        unit,
        ParameterUnit::Hz
            | ParameterUnit::Milliseconds
            | ParameterUnit::Samples
            | ParameterUnit::Ratio
    ) && value.is_finite()
        && value < 0.0
    {
        error(
            diagnostics,
            DiagnosticCode::NumericOutOfSchemaRange,
            path,
            "unit requires a non-negative value",
        );
    }
    if unit == ParameterUnit::Samples && value.is_finite() && value.fract() != 0.0 {
        error(
            diagnostics,
            DiagnosticCode::NumericOutOfSchemaRange,
            path,
            "sample values must be integral",
        );
    }
}

fn validate_finite_range(
    diagnostics: &mut Vec<Diagnostic>,
    value: f32,
    minimum: f32,
    maximum: f32,
    path: &PathRef<'_>,
) {
    if !value.is_finite() || value < minimum || value > maximum {
        error(
            diagnostics,
            if value.is_finite() {
                DiagnosticCode::NumericOutOfSchemaRange
            } else {
                DiagnosticCode::NumericNonFinite
            },
            path,
            "value is not finite f32 or outside its schema-local range",
        );
    }
}

fn validate_finite(diagnostics: &mut Vec<Diagnostic>, value: f32, path: &PathRef<'_>) {
    if !value.is_finite() {
        error(
            diagnostics,
            DiagnosticCode::NumericNonFinite,
            path,
            "value must be finite",
        );
    }
}

fn validate_nonnegative_finite(diagnostics: &mut Vec<Diagnostic>, value: f32, path: &PathRef<'_>) {
    validate_finite(diagnostics, value, path);
    if value.is_finite() && value < 0.0 {
        error(
            diagnostics,
            DiagnosticCode::NumericOutOfSchemaRange,
            path,
            "value must be non-negative",
        );
    }
}

fn error(
    diagnostics: &mut Vec<Diagnostic>,
    code: DiagnosticCode,
    path: &PathRef<'_>,
    message: &str,
) {
    if diagnostics.len() >= MAXIMUM_SESSION_DIAGNOSTICS {
        return;
    }
    diagnostics.push(Diagnostic::at(code, path, None, message));
}
