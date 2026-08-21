//! Semantic validation owned by issue 004, deliberately before graph/DSP/effect resolution.

use std::collections::BTreeSet;

use miso_engine_core::{SampleRateHz, is_launch_sample_rate};

use crate::{
    AutomationShape, Diagnostic, DiagnosticCode, DiagnosticPath, DiagnosticSet, Effect,
    MatrixOrPan, ParameterUnit, Rack, RackName, RouteDestination, RouteSource,
    SESSION_SCHEMA_VERSION_V1, SessionTomlV1, StableId,
};

pub(crate) fn validate_session(session: &SessionTomlV1) -> Result<(), DiagnosticSet> {
    let mut diagnostics = Vec::new();
    if session.schema_version != SESSION_SCHEMA_VERSION_V1 {
        error(
            &mut diagnostics,
            DiagnosticCode::VersionUnsupported,
            "$.schema_version",
            "only version 1 is accepted",
        );
    }
    if !is_launch_sample_rate(SampleRateHz(session.sample_rate_hz)) {
        error(
            &mut diagnostics,
            DiagnosticCode::SampleRateUnsupportedAtLaunch,
            "$.sample_rate_hz",
            "launch sample_rate_hz must be one of 44100, 48000, 88200, or 96000 Hz",
        );
    }
    if session.quantum_frames == 0 {
        error(
            &mut diagnostics,
            DiagnosticCode::CapacityZero,
            "$.quantum_frames",
            "quantum_frames must be nonzero",
        );
    }
    if session.output_profile.channels != 2 {
        error(
            &mut diagnostics,
            DiagnosticCode::NumericOutOfSchemaRange,
            "$.output_profile.channels",
            "V1 output must contain exactly two dual-mono channels",
        );
    }
    for (field, value) in [
        ("pcm_ring_frames", session.limits.pcm_ring_frames),
        (
            "control_queue_messages",
            session.limits.control_queue_messages,
        ),
        ("memory_bytes", session.limits.memory_bytes),
    ] {
        if value == 0 {
            error(
                &mut diagnostics,
                DiagnosticCode::CapacityZero,
                &format!("$.limits.{field}"),
                "declared capacity must be nonzero",
            );
        }
    }

    let mut source_namespace = BTreeSet::new();
    check_unique(
        &mut diagnostics,
        &mut source_namespace,
        &session.sources,
        |item| &item.id,
        "$.sources",
    );
    let mut entity_ids = BTreeSet::new();
    check_unique(
        &mut diagnostics,
        &mut entity_ids,
        &session.tracks,
        |item| &item.id,
        "$.tracks",
    );
    check_unique(
        &mut diagnostics,
        &mut entity_ids,
        &session.submixes,
        |item| &item.id,
        "$.submixes",
    );
    check_unique(
        &mut diagnostics,
        &mut entity_ids,
        &session.outputs,
        |item| &item.id,
        "$.outputs",
    );

    let source_ids: BTreeSet<_> = session.sources.iter().map(|source| &source.id).collect();
    for (index, source) in session.sources.iter().enumerate() {
        let path = format!("$.sources[{index}]");
        for (leaf, invalid) in [
            ("content.identity", source.content.identity.is_empty()),
            ("content.locator", source.content.locator.is_empty()),
            ("mapping.channel_count", source.mapping.channel_count == 0),
            (
                "mapping.region.length_samples",
                source.mapping.region.length_samples == 0,
            ),
        ] {
            if !invalid {
                continue;
            }
            error(
                &mut diagnostics,
                DiagnosticCode::NumericOutOfSchemaRange,
                &format!("{path}.{leaf}"),
                "source field must be nonzero/nonempty",
            );
        }
        if source.sample_rate_hz == 0 {
            error(
                &mut diagnostics,
                DiagnosticCode::NumericOutOfSchemaRange,
                &format!("{path}.sample_rate_hz"),
                "declared source rate must be nonzero",
            );
        }
        if source
            .mapping
            .region
            .start_sample
            .checked_add(source.mapping.region.length_samples)
            .is_none()
        {
            error(
                &mut diagnostics,
                DiagnosticCode::SourceRegionOverflow,
                &format!("{path}.mapping.region"),
                "source region endpoint overflows u64",
            );
        }
    }
    for (index, track) in session.tracks.iter().enumerate() {
        let path = format!("$.tracks[{index}]");
        if !source_ids.contains(&track.source_id) {
            error(
                &mut diagnostics,
                DiagnosticCode::MissingEntityReference,
                &format!("{path}.source_id"),
                "track source_id is not a declared source",
            );
        }
        if let Some(source) = session
            .sources
            .iter()
            .find(|source| source.id == track.source_id)
        {
            for (field, channel) in [
                ("left_source_channel", track.left_source_channel),
                ("right_source_channel", track.right_source_channel),
            ] {
                if channel >= source.mapping.channel_count {
                    error(
                        &mut diagnostics,
                        DiagnosticCode::SourceChannelIndexOutOfRange,
                        &format!("{path}.{field}"),
                        "track source channel index exceeds declared source channel_count",
                    );
                }
            }
        }
        validate_finite(
            &mut diagnostics,
            track.builtins.left.trim_db,
            &format!("{path}.builtins.left.trim_db"),
        );
        validate_finite(
            &mut diagnostics,
            track.builtins.right.trim_db,
            &format!("{path}.builtins.right.trim_db"),
        );
        for (channel, values) in [
            ("left", &track.builtins.left),
            ("right", &track.builtins.right),
        ] {
            validate_nonnegative_finite(
                &mut diagnostics,
                values.hpf_hz,
                &format!("{path}.builtins.{channel}.hpf_hz"),
            );
            validate_nonnegative_finite(
                &mut diagnostics,
                values.lpf_hz,
                &format!("{path}.builtins.{channel}.lpf_hz"),
            );
        }
        validate_finite(
            &mut diagnostics,
            track.fader.left_db,
            &format!("{path}.fader.left_db"),
        );
        validate_finite(
            &mut diagnostics,
            track.fader.right_db,
            &format!("{path}.fader.right_db"),
        );
        match track.matrix_or_pan {
            MatrixOrPan::Pan { left, right, .. } => {
                validate_finite_range(
                    &mut diagnostics,
                    left,
                    -1.0,
                    1.0,
                    &format!("{path}.pan.left"),
                );
                validate_finite_range(
                    &mut diagnostics,
                    right,
                    -1.0,
                    1.0,
                    &format!("{path}.pan.right"),
                );
            }
            MatrixOrPan::Matrix { ll, lr, rl, rr, .. } => {
                for (name, value) in [("ll", ll), ("lr", lr), ("rl", rl), ("rr", rr)] {
                    validate_finite(&mut diagnostics, value, &format!("{path}.matrix.{name}"));
                }
            }
        }
        validate_rack(
            &mut diagnostics,
            session,
            &track.simd1,
            &format!("{path}.simd1"),
        );
        validate_rack(
            &mut diagnostics,
            session,
            &track.dynamic,
            &format!("{path}.dynamic"),
        );
        validate_rack(
            &mut diagnostics,
            session,
            &track.simd2,
            &format!("{path}.simd2"),
        );
    }

    let mut route_ids = BTreeSet::new();
    for (index, route) in session.routes.iter().enumerate() {
        let path = format!("$.routes[{index}]");
        if !route_ids.insert(&route.id) {
            error(
                &mut diagnostics,
                DiagnosticCode::DuplicateId,
                &format!("{path}.id"),
                "route ID is repeated",
            );
        }
        validate_route_source(
            &mut diagnostics,
            session,
            &route.source,
            &format!("{path}.source"),
        );
        validate_route_destination(
            &mut diagnostics,
            session,
            &route.destination,
            &format!("{path}.destination"),
        );
        validate_finite(&mut diagnostics, route.gain_db, &format!("{path}.gain_db"));
        for (name, value) in [
            ("ll", route.channel_matrix.ll),
            ("lr", route.channel_matrix.lr),
            ("rl", route.channel_matrix.rl),
            ("rr", route.channel_matrix.rr),
        ] {
            validate_finite(
                &mut diagnostics,
                value,
                &format!("{path}.channel_matrix.{name}"),
            );
        }
    }

    let mut automation_ids = BTreeSet::new();
    for (index, automation) in session.automation.iter().enumerate() {
        let path = format!("$.automation[{index}]");
        if !automation_ids.insert(&automation.id) {
            error(
                &mut diagnostics,
                DiagnosticCode::DuplicateId,
                &format!("{path}.id"),
                "automation ID is repeated",
            );
        }
        if automation.segments.is_empty() {
            error(
                &mut diagnostics,
                DiagnosticCode::NumericOutOfSchemaRange,
                &format!("{path}.segments"),
                "automation must declare at least one segment",
            );
        }
        let Some(track) = session
            .tracks
            .iter()
            .find(|track| track.id == automation.target.entity_id)
        else {
            error(
                &mut diagnostics,
                DiagnosticCode::MissingEntityReference,
                &format!("{path}.target.entity_id"),
                "automation target must be a declared track",
            );
            continue;
        };
        let rack = match automation.target.rack {
            RackName::Simd1 => &track.simd1,
            RackName::Dynamic => &track.dynamic,
            RackName::Simd2 => &track.simd2,
        };
        let Some(effect) = rack
            .effects
            .iter()
            .find(|effect| effect.id == automation.target.effect_id)
        else {
            error(
                &mut diagnostics,
                DiagnosticCode::MissingEntityReference,
                &format!("{path}.target.effect_id"),
                "effect ID is absent from selected rack",
            );
            continue;
        };
        if !effect.params.iter().any(|parameter| {
            parameter.parameter_id == automation.target.parameter_id
                && parameter.channel == automation.target.channel
        }) {
            error(
                &mut diagnostics,
                DiagnosticCode::MissingEntityReference,
                &format!("{path}.target.parameter_id"),
                "parameter/channel is absent from selected effect",
            );
        }
        let mut previous_start = None;
        let mut previous_end = None;
        for (segment_index, segment) in automation.segments.iter().enumerate() {
            let segment_path = format!("{path}.segments[{segment_index}]");
            if segment.start_sample >= segment.end_sample {
                error(
                    &mut diagnostics,
                    DiagnosticCode::AutomationInvalidRange,
                    &format!("{segment_path}.end_sample"),
                    "end_sample must be greater than start_sample",
                );
            }
            if previous_start.is_some_and(|start| segment.start_sample < start) {
                error(
                    &mut diagnostics,
                    DiagnosticCode::AutomationOutOfOrder,
                    &format!("{segment_path}.start_sample"),
                    "segment starts before its predecessor",
                );
            } else if previous_end.is_some_and(|end| segment.start_sample < end) {
                error(
                    &mut diagnostics,
                    DiagnosticCode::AutomationSegmentOverlap,
                    &format!("{segment_path}.start_sample"),
                    "segment overlaps its predecessor",
                );
            }
            previous_start = Some(segment.start_sample);
            previous_end = Some(segment.end_sample);
            validate_unit_value(
                &mut diagnostics,
                segment.start_value,
                segment.unit,
                &format!("{segment_path}.start_value"),
            );
            validate_unit_value(
                &mut diagnostics,
                segment.end_value,
                segment.unit,
                &format!("{segment_path}.end_value"),
            );
            if segment.shape == AutomationShape::Exponential {
                for (field, value) in [
                    ("start_value", segment.start_value),
                    ("end_value", segment.end_value),
                ] {
                    if value.is_finite() && value <= 0.0 {
                        error(
                            &mut diagnostics,
                            DiagnosticCode::AutomationInvalidRange,
                            &format!("{segment_path}.{field}"),
                            "exponential values must be positive",
                        );
                    }
                }
            }
        }
    }
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(DiagnosticSet::from_vec(diagnostics))
    }
}

fn check_unique<T>(
    diagnostics: &mut Vec<Diagnostic>,
    ids: &mut BTreeSet<StableId>,
    values: &[T],
    id: impl Fn(&T) -> &StableId,
    path: &str,
) {
    for (index, value) in values.iter().enumerate() {
        if !ids.insert(id(value).clone()) {
            error(
                diagnostics,
                DiagnosticCode::DuplicateId,
                &format!("{path}[{index}].id"),
                "entity ID is repeated",
            );
        }
    }
}

fn validate_rack(
    diagnostics: &mut Vec<Diagnostic>,
    session: &SessionTomlV1,
    rack: &Rack,
    path: &str,
) {
    let mut ids = BTreeSet::new();
    for (index, effect) in rack.effects.iter().enumerate() {
        let effect_path = format!("{path}.effects[{index}]");
        if !ids.insert(&effect.id) {
            error(
                diagnostics,
                DiagnosticCode::DuplicateId,
                &format!("{effect_path}.id"),
                "effect ID is repeated in a rack",
            );
        }
        validate_effect(diagnostics, session, effect, &effect_path);
    }
}

fn validate_effect(
    diagnostics: &mut Vec<Diagnostic>,
    session: &SessionTomlV1,
    effect: &Effect,
    path: &str,
) {
    if let crate::EffectIdentity::ThirdPartyCid { cid } = &effect.identity
        && cid.is_empty()
    {
        error(
            diagnostics,
            DiagnosticCode::NumericOutOfSchemaRange,
            path,
            "third-party CID must be nonempty",
        );
    }
    if let crate::SidechainDeclaration::Routed(sidechain) = &effect.sidechain {
        validate_route_source(
            diagnostics,
            session,
            &sidechain.source,
            &format!("{path}.sidechain.source"),
        );
    }
    let mut parameters = BTreeSet::new();
    for (index, parameter) in effect.params.iter().enumerate() {
        let parameter_path = format!("{path}.params[{index}]");
        if !parameters.insert((&parameter.parameter_id, parameter.channel)) {
            error(
                diagnostics,
                DiagnosticCode::DuplicateId,
                &format!("{parameter_path}.parameter_id"),
                "parameter ID/channel is repeated",
            );
        }
        validate_unit_value(
            diagnostics,
            parameter.value,
            parameter.unit,
            &format!("{parameter_path}.value"),
        );
    }
}

fn validate_route_source(
    diagnostics: &mut Vec<Diagnostic>,
    session: &SessionTomlV1,
    source: &RouteSource,
    path: &str,
) {
    let (exists, leaf) = match source {
        RouteSource::Track { track_id, .. } => (
            session.tracks.iter().any(|item| item.id == *track_id),
            "track_id",
        ),
        RouteSource::SubmixOutput { submix_id } => (
            session.submixes.iter().any(|item| item.id == *submix_id),
            "submix_id",
        ),
    };
    if !exists {
        error(
            diagnostics,
            DiagnosticCode::MissingEntityReference,
            &format!("{path}.{leaf}"),
            "route source entity is not declared with the required role",
        );
    }
}

fn validate_route_destination(
    diagnostics: &mut Vec<Diagnostic>,
    session: &SessionTomlV1,
    destination: &RouteDestination,
    path: &str,
) {
    let (exists, leaf) = match destination {
        RouteDestination::SubmixInput { submix_id } => (
            session.submixes.iter().any(|item| item.id == *submix_id),
            "submix_id",
        ),
        RouteDestination::OutputInput { output_id } => (
            session.outputs.iter().any(|item| item.id == *output_id),
            "output_id",
        ),
    };
    if !exists {
        error(
            diagnostics,
            DiagnosticCode::MissingEntityReference,
            &format!("{path}.{leaf}"),
            "route destination entity is not declared with the required role",
        );
    }
}

fn validate_unit_value(
    diagnostics: &mut Vec<Diagnostic>,
    value: f32,
    unit: ParameterUnit,
    path: &str,
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
    path: &str,
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

fn validate_finite(diagnostics: &mut Vec<Diagnostic>, value: f32, path: &str) {
    if !value.is_finite() {
        error(
            diagnostics,
            DiagnosticCode::NumericNonFinite,
            path,
            "value must be finite",
        );
    }
}

fn validate_nonnegative_finite(diagnostics: &mut Vec<Diagnostic>, value: f32, path: &str) {
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

fn error(diagnostics: &mut Vec<Diagnostic>, code: DiagnosticCode, path: &str, message: &str) {
    diagnostics.push(Diagnostic::new(
        code,
        DiagnosticPath::from_dotted(path),
        None,
        message,
    ));
}
