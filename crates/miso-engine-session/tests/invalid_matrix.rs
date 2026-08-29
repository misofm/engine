//! Issue-004-owned invalid semantic and exact diagnostic-path matrix.

use miso_engine_session::{
    AutomationShape, CompileCaps, DiagnosticCode, DiagnosticSet, Effect, MatrixOrPan, Output,
    ParameterChannel, ParameterUnit, Rack, RackName, RouteDestination, RouteSource, SendTap,
    SessionToml, Sidechain, SidechainDeclaration, StableId, canonical_session_toml,
    compile_session, estimate_session_resources, parse_session_toml,
};

const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.toml");

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("valid test ID")
}

fn unlimited_caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn assert_diagnostic(error: &DiagnosticSet, code: DiagnosticCode, path: &str) {
    assert!(
        error
            .diagnostics()
            .iter()
            .any(|item| item.code == code && item.path.to_string() == path),
        "missing {code} at {path}: {error}"
    );
}

fn parse_case(count: &mut usize, source: &str, code: DiagnosticCode, path: &str) {
    let error = parse_session_toml(source).expect_err("invalid parse case");
    assert_diagnostic(&error, code, path);
    *count += 1;
}

fn model_case(
    count: &mut usize,
    mutate: impl FnOnce(&mut SessionToml),
    code: DiagnosticCode,
    path: &str,
) {
    let mut session = parse_session_toml(EXAMPLE).expect("fixture parses");
    mutate(&mut session);
    let error = canonical_session_toml(&session).expect_err("invalid model case");
    assert_diagnostic(&error, code, path);
    *count += 1;
}

#[test]
fn u64_model_fields_are_bounded_to_toml_i64_at_leaf_paths() {
    type Mutation = fn(&mut SessionToml);
    let cases: &[(Mutation, &str)] = &[
        (
            |session| session.revision = i64::MAX as u64 + 1,
            "$.revision",
        ),
        (
            |session| session.sources[0].frames = i64::MAX as u64 + 1,
            "$.sources[0].frames",
        ),
        (
            |session| session.automation[0].segments[0].start_sample = i64::MAX as u64 + 1,
            "$.automation[0].segments[0].start_sample",
        ),
        (
            |session| session.automation[0].segments[0].end_sample = i64::MAX as u64 + 1,
            "$.automation[0].segments[0].end_sample",
        ),
    ];

    for (mutate, path) in cases {
        let mut session = parse_session_toml(EXAMPLE).expect("fixture parses");
        mutate(&mut session);
        let error = canonical_session_toml(&session).expect_err("large u64 must be rejected");
        assert!(
            error.diagnostics().iter().any(|diagnostic| {
                diagnostic.code == DiagnosticCode::NumericOutOfSchemaRange
                    && diagnostic.path.to_string() == *path
                    && diagnostic.message == "integer exceeds the TOML i64 range"
            }),
            "missing TOML i64 range diagnostic at {path}: {error}"
        );
    }
}

#[test]
fn empty_third_party_cid_is_reported_at_the_identity_leaf() {
    let mut session = parse_session_toml(EXAMPLE).expect("fixture parses");
    session.tracks[0].dynamic.effects[0].identity =
        miso_engine_session::EffectIdentity::ThirdPartyCid { cid: String::new() };
    let error = canonical_session_toml(&session).expect_err("empty CID must be rejected");
    assert_diagnostic(
        &error,
        DiagnosticCode::NumericOutOfSchemaRange,
        "$.tracks[0].dynamic.effects[0].identity.cid",
    );
}

fn replaced(needle: &str, replacement: &str) -> String {
    assert!(
        EXAMPLE.contains(needle),
        "fixture replacement needle is present"
    );
    EXAMPLE.replacen(needle, replacement, 1)
}

#[test]
fn schema_version_and_type_category_has_16_distinct_cases() {
    let mut count = 0;
    parse_case(
        &mut count,
        &replaced("schema_version = 1\n", ""),
        DiagnosticCode::VersionMissing,
        "$.schema_version",
    );
    model_case(
        &mut count,
        |s| s.schema_version = 2,
        DiagnosticCode::VersionUnsupported,
        "$.schema_version",
    );
    parse_case(
        &mut count,
        "schema_version = [",
        DiagnosticCode::TomlSyntax,
        "$",
    );
    parse_case(
        &mut count,
        &replaced("schema_version = 1", "schema_version = \"1\""),
        DiagnosticCode::WrongType,
        "$.schema_version",
    );
    parse_case(
        &mut count,
        &replaced("session_id = \"demo.session\"\n", ""),
        DiagnosticCode::MissingField,
        "$.session_id",
    );
    parse_case(
        &mut count,
        &replaced("revision = 7", "revision = \"7\""),
        DiagnosticCode::WrongType,
        "$.revision",
    );
    parse_case(
        &mut count,
        &replaced("quantum_frames = 128", "quantum_frames = false"),
        DiagnosticCode::WrongType,
        "$.quantum_frames",
    );
    parse_case(
        &mut count,
        &replaced(
            "session_id = \"demo.session\"",
            "unknown_root = 1\nsession_id = \"demo.session\"",
        ),
        DiagnosticCode::UnknownField,
        "$.unknown_root",
    );
    parse_case(
        &mut count,
        &replaced("mode = \"single_thread\"", "mode = \"parallel_magic\""),
        DiagnosticCode::InvalidEnum,
        "$.render_profile.mode",
    );
    parse_case(
        &mut count,
        &replaced("sample_format = \"f32_planar\"", "sample_format = \"f64\""),
        DiagnosticCode::InvalidEnum,
        "$.output_profile.sample_format",
    );
    parse_case(
        &mut count,
        &replaced("sources = [", "sources = \"not-array\"\nold_sources = ["),
        DiagnosticCode::WrongType,
        "$.sources",
    );
    parse_case(
        &mut count,
        &replaced(
            "content = \"sha256:",
            "content = false, old_content = \"sha256:",
        ),
        DiagnosticCode::WrongType,
        "$.sources[0].content",
    );
    parse_case(
        &mut count,
        &replaced(
            "builtins = { left",
            "builtins = false, old_builtins = { left",
        ),
        DiagnosticCode::WrongType,
        "$.tracks[0].builtins",
    );
    parse_case(
        &mut count,
        &replaced("quality = \"normal\"", "quality = \"ultra\""),
        DiagnosticCode::InvalidEnum,
        "$.tracks[0].dynamic.effects[0].quality",
    );
    parse_case(
        &mut count,
        &replaced("sidechain = { kind = \"none\" }", "sidechain = false"),
        DiagnosticCode::WrongType,
        "$.tracks[0].dynamic.effects[0].sidechain",
    );
    parse_case(
        &mut count,
        &replaced(
            "source = { kind = \"track\"",
            "source = { kind = \"output\"",
        ),
        DiagnosticCode::InvalidEnum,
        "$.routes[0].source.kind",
    );
    assert_eq!(count, 16);
}

#[test]
fn stable_id_category_has_20_distinct_cases() {
    let mut count = 0;
    for (needle, replacement, path) in [
        (
            "session_id = \"demo.session\"",
            "session_id = \"Bad\"",
            "$.session_id",
        ),
        (
            "render_profile = { id = \"native\"",
            "render_profile = { id = \"Bad\"",
            "$.render_profile.id",
        ),
        (
            "output_profile = { id = \"main\"",
            "output_profile = { id = \"Bad\"",
            "$.output_profile.id",
        ),
        (
            "{ id = \"voice\", content",
            "{ id = \"Bad\", content",
            "$.sources[0].id",
        ),
        (
            "{ id = \"vocal\", source_id",
            "{ id = \"Bad\", source_id",
            "$.tracks[0].id",
        ),
        (
            "source_id = \"voice\"",
            "source_id = \"Bad\"",
            "$.tracks[0].source_id",
        ),
        (
            "[{ id = \"eq\", identity",
            "[{ id = \"Bad\", identity",
            "$.tracks[0].dynamic.effects[0].id",
        ),
        (
            "effect_id = \"parametric-eq\"",
            "effect_id = \"Bad\"",
            "$.tracks[0].dynamic.effects[0].identity.effect_id",
        ),
        (
            "{ id = \"main-out\" },",
            "{ id = \"Bad\" },",
            "$.outputs[0].id",
        ),
        (
            "{ id = \"to-main\", source",
            "{ id = \"Bad\", source",
            "$.routes[0].id",
        ),
        (
            "track_id = \"vocal\"",
            "track_id = \"Bad\"",
            "$.routes[0].source.track_id",
        ),
        (
            "output_id = \"main-out\"",
            "output_id = \"Bad\"",
            "$.routes[0].destination.output_id",
        ),
        (
            "{ id = \"eq-gain\", target",
            "{ id = \"Bad\", target",
            "$.automation[0].id",
        ),
        (
            "entity_id = \"vocal\"",
            "entity_id = \"Bad\"",
            "$.automation[0].target.entity_id",
        ),
        (
            "effect_id = \"eq\", parameter_id",
            "effect_id = \"Bad\", parameter_id",
            "$.automation[0].target.effect_id",
        ),
    ] {
        parse_case(
            &mut count,
            &replaced(needle, replacement),
            DiagnosticCode::InvalidId,
            path,
        );
    }
    parse_case(
        &mut count,
        &replaced("submixes = [\n", "submixes = [\n  { id = \"Bad\" },\n"),
        DiagnosticCode::InvalidId,
        "$.submixes[0].id",
    );
    parse_case(
        &mut count,
        &replaced(
            "sidechain = { kind = \"none\" }",
            "sidechain = { kind = \"routed\", source = { kind = \"track\", track_id = \"vocal\", tap = \"input\" }, port_id = \"Bad\" }",
        ),
        DiagnosticCode::InvalidId,
        "$.tracks[0].dynamic.effects[0].sidechain.port_id",
    );
    model_case(
        &mut count,
        |s| s.sources.push(s.sources[0].clone()),
        DiagnosticCode::DuplicateId,
        "$.sources[1].id",
    );
    model_case(
        &mut count,
        |s| {
            s.outputs.push(Output {
                id: s.tracks[0].id.clone(),
            })
        },
        DiagnosticCode::DuplicateId,
        "$.outputs[1].id",
    );
    model_case(
        &mut count,
        |s| s.routes.push(s.routes[0].clone()),
        DiagnosticCode::DuplicateId,
        "$.routes[1].id",
    );
    assert_eq!(count, 20);
}

#[test]
fn finite_unit_and_local_range_category_has_24_distinct_cases() {
    let mut count = 0;
    macro_rules! case {
        ($body:expr, $code:expr, $path:literal) => {
            model_case(&mut count, $body, $code, $path)
        };
    }
    case!(
        |s| s.tracks[0].builtins.left.trim_db = f32::NAN,
        DiagnosticCode::NumericNonFinite,
        "$.tracks[0].builtins.left.trim_db"
    );
    case!(
        |s| s.tracks[0].builtins.right.trim_db = f32::INFINITY,
        DiagnosticCode::NumericNonFinite,
        "$.tracks[0].builtins.right.trim_db"
    );
    case!(
        |s| s.tracks[0].builtins.left.hpf_hz = f32::NAN,
        DiagnosticCode::NumericNonFinite,
        "$.tracks[0].builtins.left.hpf_hz"
    );
    case!(
        |s| s.tracks[0].builtins.right.hpf_hz = f32::INFINITY,
        DiagnosticCode::NumericNonFinite,
        "$.tracks[0].builtins.right.hpf_hz"
    );
    case!(
        |s| s.tracks[0].builtins.left.lpf_hz = f32::NEG_INFINITY,
        DiagnosticCode::NumericNonFinite,
        "$.tracks[0].builtins.left.lpf_hz"
    );
    case!(
        |s| s.tracks[0].builtins.right.lpf_hz = f32::NAN,
        DiagnosticCode::NumericNonFinite,
        "$.tracks[0].builtins.right.lpf_hz"
    );
    case!(
        |s| s.tracks[0].fader.left_db = f32::NAN,
        DiagnosticCode::NumericNonFinite,
        "$.tracks[0].fader.left_db"
    );
    case!(
        |s| s.tracks[0].fader.right_db = f32::INFINITY,
        DiagnosticCode::NumericNonFinite,
        "$.tracks[0].fader.right_db"
    );
    case!(
        |s| s.tracks[0].matrix_or_pan = MatrixOrPan::Pan {
            left: f32::NAN,
            right: 0.0,
            smoothing_samples: 1
        },
        DiagnosticCode::NumericNonFinite,
        "$.tracks[0].pan.left"
    );
    case!(
        |s| s.tracks[0].matrix_or_pan = MatrixOrPan::Pan {
            left: 0.0,
            right: f32::INFINITY,
            smoothing_samples: 1
        },
        DiagnosticCode::NumericNonFinite,
        "$.tracks[0].pan.right"
    );
    case!(
        |s| s.tracks[0].matrix_or_pan = MatrixOrPan::Pan {
            left: -1.01,
            right: 0.0,
            smoothing_samples: 1
        },
        DiagnosticCode::NumericOutOfSchemaRange,
        "$.tracks[0].pan.left"
    );
    case!(
        |s| s.tracks[0].matrix_or_pan = MatrixOrPan::Pan {
            left: 0.0,
            right: 1.01,
            smoothing_samples: 1
        },
        DiagnosticCode::NumericOutOfSchemaRange,
        "$.tracks[0].pan.right"
    );
    case!(
        |s| s.routes[0].gain_db = f32::NAN,
        DiagnosticCode::NumericNonFinite,
        "$.routes[0].gain_db"
    );
    case!(
        |s| s.routes[0].channel_matrix.ll = f32::NAN,
        DiagnosticCode::NumericNonFinite,
        "$.routes[0].channel_matrix.ll"
    );
    case!(
        |s| s.routes[0].channel_matrix.lr = f32::INFINITY,
        DiagnosticCode::NumericNonFinite,
        "$.routes[0].channel_matrix.lr"
    );
    case!(
        |s| s.routes[0].channel_matrix.rl = f32::NEG_INFINITY,
        DiagnosticCode::NumericNonFinite,
        "$.routes[0].channel_matrix.rl"
    );
    case!(
        |s| s.routes[0].channel_matrix.rr = f32::NAN,
        DiagnosticCode::NumericNonFinite,
        "$.routes[0].channel_matrix.rr"
    );
    case!(
        |s| s.tracks[0].dynamic.effects[0].params[0].value = f32::NAN,
        DiagnosticCode::NumericNonFinite,
        "$.tracks[0].dynamic.effects[0].params[0].value"
    );
    case!(
        |s| {
            let p = &mut s.tracks[0].dynamic.effects[0].params[0];
            p.unit = ParameterUnit::Hz;
            p.value = -1.0;
        },
        DiagnosticCode::NumericOutOfSchemaRange,
        "$.tracks[0].dynamic.effects[0].params[0].value"
    );
    case!(
        |s| {
            let p = &mut s.tracks[0].dynamic.effects[0].params[0];
            p.unit = ParameterUnit::Samples;
            p.value = 1.5;
        },
        DiagnosticCode::NumericOutOfSchemaRange,
        "$.tracks[0].dynamic.effects[0].params[0].value"
    );
    case!(
        |s| s.automation[0].segments[0].start_value = f32::NAN,
        DiagnosticCode::NumericNonFinite,
        "$.automation[0].segments[0].start_value"
    );
    case!(
        |s| s.automation[0].segments[0].end_value = f32::INFINITY,
        DiagnosticCode::NumericNonFinite,
        "$.automation[0].segments[0].end_value"
    );
    case!(
        |s| {
            let p = &mut s.automation[0].segments[0];
            p.unit = ParameterUnit::Ratio;
            p.start_value = -0.1;
        },
        DiagnosticCode::NumericOutOfSchemaRange,
        "$.automation[0].segments[0].start_value"
    );
    case!(
        |s| {
            let p = &mut s.automation[0].segments[0];
            p.unit = ParameterUnit::Samples;
            p.end_value = 1.5;
        },
        DiagnosticCode::NumericOutOfSchemaRange,
        "$.automation[0].segments[0].end_value"
    );
    assert_eq!(count, 24);
}

#[test]
fn source_identity_and_shape_category_has_20_distinct_cases() {
    let mut count = 0;
    model_case(
        &mut count,
        |s| s.sources[0].content.clear(),
        DiagnosticCode::SourceContentIdentityFormat,
        "$.sources[0].content",
    );
    model_case(
        &mut count,
        |s| s.sources[0].content = "sha256:abc".to_owned(),
        DiagnosticCode::SourceContentIdentityFormat,
        "$.sources[0].content",
    );
    model_case(
        &mut count,
        |s| {
            s.sources[0].content[7..].make_ascii_uppercase();
        },
        DiagnosticCode::SourceContentIdentityFormat,
        "$.sources[0].content",
    );
    model_case(
        &mut count,
        |s| s.sources[0].content = s.sources[0].content.replacen("sha256:", "sha512:", 1),
        DiagnosticCode::SourceContentIdentityFormat,
        "$.sources[0].content",
    );
    model_case(
        &mut count,
        |s| s.sources[0].content.replace_range(7..8, "g"),
        DiagnosticCode::SourceContentIdentityFormat,
        "$.sources[0].content",
    );
    model_case(
        &mut count,
        |s| s.sources[0].channels = 0,
        DiagnosticCode::CapacityZero,
        "$.sources[0].channels",
    );
    model_case(
        &mut count,
        |s| s.sources[0].frames = 0,
        DiagnosticCode::CapacityZero,
        "$.sources[0].frames",
    );
    model_case(
        &mut count,
        |s| s.tracks[0].left_source_channel = 2,
        DiagnosticCode::SourceChannelIndexOutOfRange,
        "$.tracks[0].left_source_channel",
    );
    model_case(
        &mut count,
        |s| s.tracks[0].right_source_channel = 2,
        DiagnosticCode::SourceChannelIndexOutOfRange,
        "$.tracks[0].right_source_channel",
    );
    for (needle, replacement, path) in [
        (
            "channels = 2, bit_depth",
            "channels = -1, bit_depth",
            "$.sources[0].channels",
        ),
        (
            "channels = 2, bit_depth",
            "channels = 256, bit_depth",
            "$.sources[0].channels",
        ),
        ("frames = 48000", "frames = -1", "$.sources[0].frames"),
        (
            "left_source_channel = 0",
            "left_source_channel = 256",
            "$.tracks[0].left_source_channel",
        ),
        (
            "right_source_channel = 1",
            "right_source_channel = -1",
            "$.tracks[0].right_source_channel",
        ),
        (
            "right_source_channel = 1",
            "right_source_channel = 256",
            "$.tracks[0].right_source_channel",
        ),
    ] {
        parse_case(
            &mut count,
            &replaced(needle, replacement),
            DiagnosticCode::NumericOutOfSchemaRange,
            path,
        );
    }
    // The closed depth set is {16, 24, "32f"} and each token has exactly one legal spelling.
    // Integer 32 is the trap: it is the obvious "f32" spelling, it is not in the set, and nothing
    // pinned its refusal until this row -- widening the integer arm to map 32 onto `Float32`
    // passed the whole suite. The string spelling of an integer depth and a zero depth close the
    // two remaining spelling holes on the same axis.
    for replacement in [
        "bit_depth = \"20f\"",
        "bit_depth = 32",
        "bit_depth = \"16\"",
        "bit_depth = \"24\"",
        "bit_depth = 0",
    ] {
        parse_case(
            &mut count,
            &replaced("bit_depth = \"32f\"", replacement),
            DiagnosticCode::SourceBitDepthUnsupported,
            "$.sources[0].bit_depth",
        );
    }
    assert_eq!(count, 20);
}

fn routed_effect(template: &Effect, source: RouteSource) -> Effect {
    let mut effect = template.clone();
    effect.sidechain = SidechainDeclaration::Routed(Sidechain {
        source,
        port_id: id("detector-in"),
    });
    effect
}

#[test]
fn schema_owned_reference_category_has_20_distinct_cases() {
    let mut count = 0;
    model_case(
        &mut count,
        |s| s.tracks[0].source_id = id("missing-source-a"),
        DiagnosticCode::MissingEntityReference,
        "$.tracks[0].source_id",
    );
    model_case(
        &mut count,
        |s| {
            let mut t = s.tracks[0].clone();
            t.id = id("second-track");
            t.source_id = id("missing-source-b");
            s.tracks.push(t);
        },
        DiagnosticCode::MissingEntityReference,
        "$.tracks[1].source_id",
    );
    model_case(
        &mut count,
        |s| {
            s.routes[0].source = RouteSource::Track {
                track_id: id("missing-track-a"),
                tap: SendTap::Input,
            }
        },
        DiagnosticCode::MissingEntityReference,
        "$.routes[0].source.track_id",
    );
    model_case(
        &mut count,
        |s| {
            let mut r = s.routes[0].clone();
            r.id = id("route-b");
            r.source = RouteSource::Track {
                track_id: id("missing-track-b"),
                tap: SendTap::PostFader,
            };
            s.routes.push(r);
        },
        DiagnosticCode::MissingEntityReference,
        "$.routes[1].source.track_id",
    );
    model_case(
        &mut count,
        |s| {
            s.routes[0].source = RouteSource::SubmixOutput {
                submix_id: id("missing-mix-a"),
            }
        },
        DiagnosticCode::MissingEntityReference,
        "$.routes[0].source.submix_id",
    );
    model_case(
        &mut count,
        |s| {
            let mut r = s.routes[0].clone();
            r.id = id("route-b");
            r.source = RouteSource::SubmixOutput {
                submix_id: id("missing-mix-b"),
            };
            s.routes.push(r);
        },
        DiagnosticCode::MissingEntityReference,
        "$.routes[1].source.submix_id",
    );
    model_case(
        &mut count,
        |s| {
            s.routes[0].destination = RouteDestination::OutputInput {
                output_id: id("missing-output-a"),
            }
        },
        DiagnosticCode::MissingEntityReference,
        "$.routes[0].destination.output_id",
    );
    model_case(
        &mut count,
        |s| {
            let mut r = s.routes[0].clone();
            r.id = id("route-b");
            r.destination = RouteDestination::OutputInput {
                output_id: id("missing-output-b"),
            };
            s.routes.push(r);
        },
        DiagnosticCode::MissingEntityReference,
        "$.routes[1].destination.output_id",
    );
    model_case(
        &mut count,
        |s| {
            s.routes[0].destination = RouteDestination::SubmixInput {
                submix_id: id("missing-mix-a"),
            }
        },
        DiagnosticCode::MissingEntityReference,
        "$.routes[0].destination.submix_id",
    );
    model_case(
        &mut count,
        |s| {
            let mut r = s.routes[0].clone();
            r.id = id("route-b");
            r.destination = RouteDestination::SubmixInput {
                submix_id: id("missing-mix-b"),
            };
            s.routes.push(r);
        },
        DiagnosticCode::MissingEntityReference,
        "$.routes[1].destination.submix_id",
    );
    for (rack, source, path) in [
        (
            RackName::Simd1,
            RouteSource::Track {
                track_id: id("missing-a"),
                tap: SendTap::Input,
            },
            "$.tracks[0].simd1.effects[0].sidechain.source.track_id",
        ),
        (
            RackName::Dynamic,
            RouteSource::Track {
                track_id: id("missing-b"),
                tap: SendTap::PostFader,
            },
            "$.tracks[0].dynamic.effects[0].sidechain.source.track_id",
        ),
        (
            RackName::Simd2,
            RouteSource::Track {
                track_id: id("missing-c"),
                tap: SendTap::PostMatrix,
            },
            "$.tracks[0].simd2.effects[0].sidechain.source.track_id",
        ),
        (
            RackName::Simd1,
            RouteSource::SubmixOutput {
                submix_id: id("missing-d"),
            },
            "$.tracks[0].simd1.effects[0].sidechain.source.submix_id",
        ),
        (
            RackName::Dynamic,
            RouteSource::SubmixOutput {
                submix_id: id("missing-e"),
            },
            "$.tracks[0].dynamic.effects[0].sidechain.source.submix_id",
        ),
        (
            RackName::Simd2,
            RouteSource::SubmixOutput {
                submix_id: id("missing-f"),
            },
            "$.tracks[0].simd2.effects[0].sidechain.source.submix_id",
        ),
    ] {
        model_case(
            &mut count,
            |s| {
                let effect = routed_effect(&s.tracks[0].dynamic.effects[0], source);
                match rack {
                    RackName::Simd1 => {
                        s.tracks[0].simd1 = Rack {
                            effects: vec![effect],
                        }
                    }
                    RackName::Dynamic => s.tracks[0].dynamic.effects[0] = effect,
                    RackName::Simd2 => {
                        s.tracks[0].simd2 = Rack {
                            effects: vec![effect],
                        }
                    }
                    // The strip carries no effects; the case table above never names it.
                    RackName::Builtins => {
                        unreachable!("the builtins token addresses no effect rack")
                    }
                }
            },
            DiagnosticCode::MissingEntityReference,
            path,
        );
    }
    model_case(
        &mut count,
        |s| s.automation[0].target.entity_id = id("missing-track"),
        DiagnosticCode::MissingEntityReference,
        "$.automation[0].target.entity_id",
    );
    model_case(
        &mut count,
        |s| s.automation[0].target.effect_id = id("missing-effect"),
        DiagnosticCode::MissingEntityReference,
        "$.automation[0].target.effect_id",
    );
    model_case(
        &mut count,
        |s| s.automation[0].target.parameter_id = 999,
        DiagnosticCode::MissingEntityReference,
        "$.automation[0].target.parameter_id",
    );
    model_case(
        &mut count,
        |s| s.automation[0].target.channel = ParameterChannel::Left,
        DiagnosticCode::MissingEntityReference,
        "$.automation[0].target.parameter_id",
    );
    assert_eq!(count, 20);
}

#[test]
fn automation_category_has_20_distinct_cases() {
    let mut count = 0;
    model_case(
        &mut count,
        |s| s.automation.push(s.automation[0].clone()),
        DiagnosticCode::DuplicateId,
        "$.automation[1].id",
    );
    model_case(
        &mut count,
        |s| s.automation[0].segments.clear(),
        DiagnosticCode::NumericOutOfSchemaRange,
        "$.automation[0].segments",
    );
    model_case(
        &mut count,
        |s| s.automation[0].segments[0].end_sample = 0,
        DiagnosticCode::AutomationInvalidRange,
        "$.automation[0].segments[0].end_sample",
    );
    model_case(
        &mut count,
        |s| {
            s.automation[0].segments[0].start_sample = 2;
            s.automation[0].segments[0].end_sample = 1;
        },
        DiagnosticCode::AutomationInvalidRange,
        "$.automation[0].segments[0].end_sample",
    );
    model_case(
        &mut count,
        |s| {
            let mut p = s.automation[0].segments[0].clone();
            p.start_sample = 479;
            p.end_sample = 960;
            s.automation[0].segments.push(p);
        },
        DiagnosticCode::AutomationSegmentOverlap,
        "$.automation[0].segments[1].start_sample",
    );
    model_case(
        &mut count,
        |s| {
            s.automation[0].segments[0].start_sample = 500;
            s.automation[0].segments[0].end_sample = 600;
            let mut p = s.automation[0].segments[0].clone();
            p.start_sample = 100;
            p.end_sample = 200;
            s.automation[0].segments.push(p);
        },
        DiagnosticCode::AutomationOutOfOrder,
        "$.automation[0].segments[1].start_sample",
    );
    for (start, end, path) in [
        (0.0, 1.0, "$.automation[0].segments[0].start_value"),
        (1.0, 0.0, "$.automation[0].segments[0].end_value"),
        (-1.0, 1.0, "$.automation[0].segments[0].start_value"),
        (1.0, -1.0, "$.automation[0].segments[0].end_value"),
    ] {
        model_case(
            &mut count,
            |s| {
                let p = &mut s.automation[0].segments[0];
                p.shape = AutomationShape::Exponential;
                p.start_value = start;
                p.end_value = end;
            },
            DiagnosticCode::AutomationInvalidRange,
            path,
        );
    }
    model_case(
        &mut count,
        |s| s.automation[0].segments[0].start_value = f32::NAN,
        DiagnosticCode::NumericNonFinite,
        "$.automation[0].segments[0].start_value",
    );
    model_case(
        &mut count,
        |s| s.automation[0].segments[0].end_value = f32::INFINITY,
        DiagnosticCode::NumericNonFinite,
        "$.automation[0].segments[0].end_value",
    );
    model_case(
        &mut count,
        |s| {
            let p = &mut s.automation[0].segments[0];
            p.unit = ParameterUnit::Hz;
            p.start_value = -1.0;
        },
        DiagnosticCode::NumericOutOfSchemaRange,
        "$.automation[0].segments[0].start_value",
    );
    model_case(
        &mut count,
        |s| {
            let p = &mut s.automation[0].segments[0];
            p.unit = ParameterUnit::Milliseconds;
            p.end_value = -1.0;
        },
        DiagnosticCode::NumericOutOfSchemaRange,
        "$.automation[0].segments[0].end_value",
    );
    model_case(
        &mut count,
        |s| {
            let p = &mut s.automation[0].segments[0];
            p.unit = ParameterUnit::Samples;
            p.start_value = 1.5;
        },
        DiagnosticCode::NumericOutOfSchemaRange,
        "$.automation[0].segments[0].start_value",
    );
    model_case(
        &mut count,
        |s| {
            let p = &mut s.automation[0].segments[0];
            p.unit = ParameterUnit::Samples;
            p.end_value = 2.5;
        },
        DiagnosticCode::NumericOutOfSchemaRange,
        "$.automation[0].segments[0].end_value",
    );
    parse_case(
        &mut count,
        &replaced("shape = \"linear\"", "shape = \"curve\""),
        DiagnosticCode::InvalidEnum,
        "$.automation[0].segments[0].shape",
    );
    parse_case(
        &mut count,
        &replaced("rack = \"dynamic\"", "rack = \"master\""),
        DiagnosticCode::InvalidEnum,
        "$.automation[0].target.rack",
    );
    parse_case(
        &mut count,
        &replaced(
            "channel = \"both\" }, segments",
            "channel = \"middle\" }, segments",
        ),
        DiagnosticCode::InvalidEnum,
        "$.automation[0].target.channel",
    );
    parse_case(
        &mut count,
        &replaced("unit = \"db\" }] },", "unit = \"seconds\" }] },"),
        DiagnosticCode::UnitInvalid,
        "$.automation[0].segments[0].unit",
    );
    assert_eq!(count, 20);
}

#[test]
fn configured_resource_category_has_4_distinct_cases() {
    let mut count = 0;
    let base = parse_session_toml(EXAMPLE).expect("fixture parses");
    let estimate = estimate_session_resources(&base).expect("fixture estimates");
    for zero in [true, false] {
        for (field, path) in [
            ("compiled", "$.compile_caps.max_compiled_model_bytes"),
            ("single", "$.compile_caps.max_single_allocation_bytes"),
        ] {
            let mut caps = unlimited_caps();
            let limit = match field {
                "compiled" => estimate.compiled_model_bytes,
                "single" => estimate.single_allocation_bytes,
                _ => unreachable!(),
            };
            let rejected = if zero { 0 } else { limit - 1 };
            match field {
                "compiled" => caps.max_compiled_model_bytes = rejected,
                "single" => caps.max_single_allocation_bytes = rejected,
                _ => unreachable!(),
            }
            let error = compile_session(&base, caps).expect_err("configured cap rejects");
            assert_diagnostic(&error, DiagnosticCode::ResourceLimitExceeded, path);
            count += 1;
        }
    }
    assert_eq!(count, 4);
}

/// The four resource caps #241 made inert cannot refuse any session, in either direction.
///
/// `configured_resource_category` dropped from 16 rows to 4 because these four stopped being
/// reachable refusal channels; nothing then stated *that* as a fact, so the shrink read as lost
/// coverage rather than as a deliberate hand-off. This row states it.
///
/// Set all four to zero -- the most hostile budget expressible -- against a session that really
/// does declare sources and effects, and compilation must still succeed. The two live caps are
/// left unlimited so the only thing under test is the inertness of the other four.
///
/// If a future issue re-populates the estimate (#240 S3.7 recomputing ring bytes after the host's
/// choice is the obvious candidate), this row goes red and forces the question rather than
/// letting a budget quietly start or stop biting.
#[test]
fn dead_resource_caps_cannot_refuse_any_session() {
    let session = parse_session_toml(EXAMPLE).expect("fixture parses");
    let estimate = estimate_session_resources(&session).expect("fixture estimates");
    for (label, value) in [
        ("requested_runtime_bytes", estimate.requested_runtime_bytes),
        ("queue_items", estimate.queue_items),
        ("queue_bytes", estimate.queue_bytes),
        ("source_ring_frames", estimate.source_ring_frames),
        ("source_ring_bytes", estimate.source_ring_bytes),
    ] {
        assert_eq!(
            value, 0,
            "{label} must be the host's business, not the document's"
        );
    }
    let caps = CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_requested_runtime_bytes: 0,
        max_queue_items: 0,
        max_source_ring_frames: 0,
        max_source_ring_bytes: 0,
    };
    compile_session(&session, caps)
        .expect("caps whose subjects are host policy cannot refuse a session");
}

#[test]
fn corpus_distribution_totals_124_cases() {
    const DISTRIBUTION: [usize; 7] = [16, 20, 24, 20, 20, 20, 4];
    assert_eq!(DISTRIBUTION.iter().sum::<usize>(), 124);
}

#[test]
fn parser_registry_covers_right_lane_and_f32_representation() {
    let unknown_right = replaced(
        "right = { polarity_invert = false",
        "right = { unknown_right = 0, polarity_invert = false",
    );
    assert_diagnostic(
        &parse_session_toml(&unknown_right).expect_err("nested unknown"),
        DiagnosticCode::UnknownField,
        "$.tracks[0].builtins.right.unknown_right",
    );
    let too_large = replaced("trim_db = 0.0", "trim_db = 3.5e38");
    assert_diagnostic(
        &parse_session_toml(&too_large).expect_err("finite f64 does not fit finite f32"),
        DiagnosticCode::NumericNotF32Representable,
        "$.tracks[0].builtins.left.trim_db",
    );
}

#[test]
fn compile_caps_reject_before_semantic_validation() {
    let mut session = parse_session_toml(EXAMPLE).expect("fixture parses");
    session.tracks[0].source_id = id("missing-source");
    let mut caps = unlimited_caps();
    caps.max_compiled_model_bytes = 1;
    let error = compile_session(&session, caps).expect_err("preflight cap rejects");
    assert!(
        error
            .diagnostics()
            .iter()
            .all(|item| item.code == DiagnosticCode::ResourceLimitExceeded)
    );
}

#[test]
fn tagged_route_roles_are_structurally_closed() {
    let session = parse_session_toml(EXAMPLE).expect("fixture parses");
    assert!(matches!(
        session.routes[0].source,
        RouteSource::Track { .. } | RouteSource::SubmixOutput { .. }
    ));
    assert!(matches!(
        session.routes[0].destination,
        RouteDestination::SubmixInput { .. } | RouteDestination::OutputInput { .. }
    ));
}
