//! E5: every text-expressible validator mutation has text/typed entry-point parity.

use core::{convert::Infallible, fmt::Write as _};
use session::{
    AutomationShape, CompileCaps, DiagnosticCode, DiagnosticSet, FieldKey, MatrixOrPan,
    ModelVisitor, Output, ParameterChannel, ParameterUnit, Rack, RackName, RouteDestination,
    RouteSource, SendTap, SessionToml, Sidechain, SidechainDeclaration, StableId, Token,
    VisitModel, WalkOrder, canonical_session_toml, compile_session, parse_session_toml,
};

const CANONICAL: &str = include_str!("../../../fixtures/session/v1/canonical.toml");
fn caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}
fn id(text: &str) -> StableId {
    StableId::parse(text).expect("test ID")
}

struct Case {
    name: &'static str,
    code: DiagnosticCode,
    path: &'static str,
    typed: fn(&mut SessionToml),
}
fn target<'a>(errors: &'a DiagnosticSet, case: &Case) -> &'a session::Diagnostic {
    errors
        .diagnostics()
        .iter()
        .find(|item| item.code == case.code && item.path.to_string() == case.path)
        .unwrap_or_else(|| {
            panic!(
                "{} missing {} at {}: {errors}",
                case.name, case.code, case.path
            )
        })
}
fn set_sidechain(model: &mut SessionToml, rack: RackName, source: RouteSource) {
    let mut effect = model.tracks[0].dynamic.effects[0].clone();
    effect.sidechain = SidechainDeclaration::Routed(Sidechain {
        source,
        port_id: id("detector-in"),
    });
    match rack {
        RackName::Simd1 => {
            model.tracks[0].simd1 = Rack {
                effects: vec![effect],
            }
        }
        RackName::Dynamic => model.tracks[0].dynamic.effects[0] = effect,
        RackName::Simd2 => {
            model.tracks[0].simd2 = Rack {
                effects: vec![effect],
            }
        }
        // The strip carries no effects and so no sidechain; the cases below never name it.
        RackName::Builtins => unreachable!("the builtins token addresses no effect rack"),
    }
}
macro_rules! case {
    ($name:literal,$code:ident,$path:literal,$typed:expr) => {
        Case {
            name: $name,
            code: DiagnosticCode::$code,
            path: $path,
            typed: $typed,
        }
    };
}

#[test]
fn parse_canonical_and_compile_diagnostics_have_code_path_and_span_parity() {
    let cases = [
        case!("version", VersionUnsupported, "$.schema_version", |s| s
            .schema_version =
            2),
        case!(
            "sample-rate",
            SampleRateUnsupportedAtLaunch,
            "$.sample_rate_hz",
            |s| s.sample_rate_hz = 192_000
        ),
        case!("quantum", CapacityZero, "$.quantum_frames", |s| s
            .quantum_frames =
            0),
        case!(
            "output-channels",
            NumericOutOfSchemaRange,
            "$.output_profile.channels",
            |s| s.output_profile.channels = 1
        ),
        case!("revision-i64", NumericOutOfSchemaRange, "$.revision", |s| {
            s.revision = i64::MAX as u64 + 1
        }),
        case!(
            "source-frames-i64",
            NumericOutOfSchemaRange,
            "$.sources[0].frames",
            |s| s.sources[0].frames = i64::MAX as u64 + 1
        ),
        case!(
            "automation-start-i64",
            NumericOutOfSchemaRange,
            "$.automation[0].segments[0].start_sample",
            |s| s.automation[0].segments[0].start_sample = i64::MAX as u64 + 1
        ),
        case!(
            "automation-end-i64",
            NumericOutOfSchemaRange,
            "$.automation[0].segments[0].end_sample",
            |s| s.automation[0].segments[0].end_sample = i64::MAX as u64 + 1
        ),
        case!(
            "empty-cid",
            NumericOutOfSchemaRange,
            "$.tracks[0].dynamic.effects[0].identity.cid",
            |s| s.tracks[0].dynamic.effects[0].identity =
                session::EffectIdentity::ThirdPartyCid { cid: String::new() }
        ),
        case!("duplicate-source", DuplicateId, "$.sources[1].id", |s| s
            .sources
            .push(s.sources[0].clone())),
        case!("duplicate-graph", DuplicateId, "$.outputs[1].id", |s| s
            .outputs
            .push(Output {
                id: s.tracks[0].id.clone()
            })),
        case!("duplicate-route", DuplicateId, "$.routes[1].id", |s| s
            .routes
            .push(s.routes[0].clone())),
        case!(
            "left-trim-finite",
            NumericNonFinite,
            "$.tracks[0].builtins.left.trim_db",
            |s| s.tracks[0].builtins.left.trim_db = f32::NAN
        ),
        case!(
            "right-trim-finite",
            NumericNonFinite,
            "$.tracks[0].builtins.right.trim_db",
            |s| s.tracks[0].builtins.right.trim_db = f32::INFINITY
        ),
        case!(
            "left-hpf-finite",
            NumericNonFinite,
            "$.tracks[0].builtins.left.hpf_hz",
            |s| s.tracks[0].builtins.left.hpf_hz = f32::NAN
        ),
        case!(
            "right-hpf-finite",
            NumericNonFinite,
            "$.tracks[0].builtins.right.hpf_hz",
            |s| s.tracks[0].builtins.right.hpf_hz = f32::INFINITY
        ),
        case!(
            "left-lpf-finite",
            NumericNonFinite,
            "$.tracks[0].builtins.left.lpf_hz",
            |s| s.tracks[0].builtins.left.lpf_hz = f32::NEG_INFINITY
        ),
        case!(
            "right-lpf-finite",
            NumericNonFinite,
            "$.tracks[0].builtins.right.lpf_hz",
            |s| s.tracks[0].builtins.right.lpf_hz = f32::NAN
        ),
        case!(
            "left-fader-finite",
            NumericNonFinite,
            "$.tracks[0].fader.left_db",
            |s| s.tracks[0].fader.left_db = f32::NAN
        ),
        case!(
            "right-fader-finite",
            NumericNonFinite,
            "$.tracks[0].fader.right_db",
            |s| s.tracks[0].fader.right_db = f32::INFINITY
        ),
        case!(
            "pan-left-finite",
            NumericNonFinite,
            "$.tracks[0].pan.left",
            |s| s.tracks[0].matrix_or_pan = MatrixOrPan::Pan {
                left: f32::NAN,
                right: 0.0,
                smoothing_samples: 1
            }
        ),
        case!(
            "pan-right-finite",
            NumericNonFinite,
            "$.tracks[0].pan.right",
            |s| s.tracks[0].matrix_or_pan = MatrixOrPan::Pan {
                left: 0.0,
                right: f32::INFINITY,
                smoothing_samples: 1
            }
        ),
        case!(
            "pan-left-range",
            NumericOutOfSchemaRange,
            "$.tracks[0].pan.left",
            |s| s.tracks[0].matrix_or_pan = MatrixOrPan::Pan {
                left: -1.01,
                right: 0.0,
                smoothing_samples: 1
            }
        ),
        case!(
            "pan-right-range",
            NumericOutOfSchemaRange,
            "$.tracks[0].pan.right",
            |s| s.tracks[0].matrix_or_pan = MatrixOrPan::Pan {
                left: 0.0,
                right: 1.01,
                smoothing_samples: 1
            }
        ),
        case!(
            "route-gain-finite",
            NumericNonFinite,
            "$.routes[0].gain_db",
            |s| s.routes[0].gain_db = f32::NAN
        ),
        case!(
            "route-ll-finite",
            NumericNonFinite,
            "$.routes[0].channel_matrix.ll",
            |s| s.routes[0].channel_matrix.ll = f32::NAN
        ),
        case!(
            "route-lr-finite",
            NumericNonFinite,
            "$.routes[0].channel_matrix.lr",
            |s| s.routes[0].channel_matrix.lr = f32::INFINITY
        ),
        case!(
            "route-rl-finite",
            NumericNonFinite,
            "$.routes[0].channel_matrix.rl",
            |s| s.routes[0].channel_matrix.rl = f32::NEG_INFINITY
        ),
        case!(
            "route-rr-finite",
            NumericNonFinite,
            "$.routes[0].channel_matrix.rr",
            |s| s.routes[0].channel_matrix.rr = f32::NAN
        ),
        case!(
            "param-finite",
            NumericNonFinite,
            "$.tracks[0].dynamic.effects[0].params[0].value",
            |s| s.tracks[0].dynamic.effects[0].params[0].value = f32::NAN
        ),
        case!(
            "param-negative",
            NumericOutOfSchemaRange,
            "$.tracks[0].dynamic.effects[0].params[0].value",
            |s| {
                let p = &mut s.tracks[0].dynamic.effects[0].params[0];
                p.unit = ParameterUnit::Hz;
                p.value = -1.0
            }
        ),
        case!(
            "param-fraction",
            NumericOutOfSchemaRange,
            "$.tracks[0].dynamic.effects[0].params[0].value",
            |s| {
                let p = &mut s.tracks[0].dynamic.effects[0].params[0];
                p.unit = ParameterUnit::Samples;
                p.value = 1.5
            }
        ),
        case!(
            "automation-start-finite",
            NumericNonFinite,
            "$.automation[0].segments[0].start_value",
            |s| s.automation[0].segments[0].start_value = f32::NAN
        ),
        case!(
            "automation-end-finite",
            NumericNonFinite,
            "$.automation[0].segments[0].end_value",
            |s| s.automation[0].segments[0].end_value = f32::INFINITY
        ),
        case!(
            "automation-start-negative",
            NumericOutOfSchemaRange,
            "$.automation[0].segments[0].start_value",
            |s| {
                let p = &mut s.automation[0].segments[0];
                p.unit = ParameterUnit::Ratio;
                p.start_value = -0.1
            }
        ),
        case!(
            "automation-end-fraction",
            NumericOutOfSchemaRange,
            "$.automation[0].segments[0].end_value",
            |s| {
                let p = &mut s.automation[0].segments[0];
                p.unit = ParameterUnit::Samples;
                p.end_value = 1.5
            }
        ),
        case!(
            "source-content-empty",
            SourceContentIdentityFormat,
            "$.sources[0].content",
            |s| s.sources[0].content.clear()
        ),
        case!(
            "source-content-short",
            SourceContentIdentityFormat,
            "$.sources[0].content",
            |s| s.sources[0].content = "sha256:abc".to_owned()
        ),
        case!(
            "source-content-uppercase",
            SourceContentIdentityFormat,
            "$.sources[0].content",
            |s| {
                s.sources[0].content[7..].make_ascii_uppercase();
            }
        ),
        case!(
            "source-content-nonhex",
            SourceContentIdentityFormat,
            "$.sources[0].content",
            |s| s.sources[0].content.replace_range(7..8, "g")
        ),
        case!(
            "source-channels",
            CapacityZero,
            "$.sources[0].channels",
            |s| s.sources[0].channels = 0
        ),
        case!("source-frames", CapacityZero, "$.sources[0].frames", |s| {
            s.sources[0].frames = 0
        }),
        case!(
            "left-source-channel",
            SourceChannelIndexOutOfRange,
            "$.tracks[0].left_source_channel",
            |s| s.tracks[0].left_source_channel = 2
        ),
        case!(
            "right-source-channel",
            SourceChannelIndexOutOfRange,
            "$.tracks[0].right_source_channel",
            |s| s.tracks[0].right_source_channel = 2
        ),
        case!(
            "track-source-a",
            MissingEntityReference,
            "$.tracks[0].source_id",
            |s| s.tracks[0].source_id = id("missing-source-a")
        ),
        case!(
            "track-source-b",
            MissingEntityReference,
            "$.tracks[1].source_id",
            |s| {
                let mut t = s.tracks[0].clone();
                t.id = id("second-track");
                t.source_id = id("missing-source-b");
                s.tracks.push(t)
            }
        ),
        case!(
            "route-track-a",
            MissingEntityReference,
            "$.routes[0].source.track_id",
            |s| s.routes[0].source = RouteSource::Track {
                track_id: id("missing-track-a"),
                tap: SendTap::Input
            }
        ),
        case!(
            "route-track-b",
            MissingEntityReference,
            "$.routes[1].source.track_id",
            |s| {
                let mut r = s.routes[0].clone();
                r.id = id("route-b");
                r.source = RouteSource::Track {
                    track_id: id("missing-track-b"),
                    tap: SendTap::PostFader,
                };
                s.routes.push(r)
            }
        ),
        case!(
            "route-submix-a",
            MissingEntityReference,
            "$.routes[0].source.submix_id",
            |s| s.routes[0].source = RouteSource::SubmixOutput {
                submix_id: id("missing-mix-a")
            }
        ),
        case!(
            "route-submix-b",
            MissingEntityReference,
            "$.routes[1].source.submix_id",
            |s| {
                let mut r = s.routes[0].clone();
                r.id = id("route-b");
                r.source = RouteSource::SubmixOutput {
                    submix_id: id("missing-mix-b"),
                };
                s.routes.push(r)
            }
        ),
        case!(
            "destination-output-a",
            MissingEntityReference,
            "$.routes[0].destination.output_id",
            |s| s.routes[0].destination = RouteDestination::OutputInput {
                output_id: id("missing-output-a")
            }
        ),
        case!(
            "destination-output-b",
            MissingEntityReference,
            "$.routes[1].destination.output_id",
            |s| {
                let mut r = s.routes[0].clone();
                r.id = id("route-b");
                r.destination = RouteDestination::OutputInput {
                    output_id: id("missing-output-b"),
                };
                s.routes.push(r)
            }
        ),
        case!(
            "destination-submix-a",
            MissingEntityReference,
            "$.routes[0].destination.submix_id",
            |s| s.routes[0].destination = RouteDestination::SubmixInput {
                submix_id: id("missing-mix-a")
            }
        ),
        case!(
            "destination-submix-b",
            MissingEntityReference,
            "$.routes[1].destination.submix_id",
            |s| {
                let mut r = s.routes[0].clone();
                r.id = id("route-b");
                r.destination = RouteDestination::SubmixInput {
                    submix_id: id("missing-mix-b"),
                };
                s.routes.push(r)
            }
        ),
        case!(
            "simd1-sidechain-track",
            MissingEntityReference,
            "$.tracks[0].simd1.effects[0].sidechain.source.track_id",
            |s| set_sidechain(
                s,
                RackName::Simd1,
                RouteSource::Track {
                    track_id: id("missing-a"),
                    tap: SendTap::Input
                }
            )
        ),
        case!(
            "dynamic-sidechain-track",
            MissingEntityReference,
            "$.tracks[0].dynamic.effects[0].sidechain.source.track_id",
            |s| set_sidechain(
                s,
                RackName::Dynamic,
                RouteSource::Track {
                    track_id: id("missing-b"),
                    tap: SendTap::PostFader
                }
            )
        ),
        case!(
            "simd2-sidechain-track",
            MissingEntityReference,
            "$.tracks[0].simd2.effects[0].sidechain.source.track_id",
            |s| set_sidechain(
                s,
                RackName::Simd2,
                RouteSource::Track {
                    track_id: id("missing-c"),
                    tap: SendTap::PostMatrix
                }
            )
        ),
        case!(
            "simd1-sidechain-submix",
            MissingEntityReference,
            "$.tracks[0].simd1.effects[0].sidechain.source.submix_id",
            |s| set_sidechain(
                s,
                RackName::Simd1,
                RouteSource::SubmixOutput {
                    submix_id: id("missing-d")
                }
            )
        ),
        case!(
            "dynamic-sidechain-submix",
            MissingEntityReference,
            "$.tracks[0].dynamic.effects[0].sidechain.source.submix_id",
            |s| set_sidechain(
                s,
                RackName::Dynamic,
                RouteSource::SubmixOutput {
                    submix_id: id("missing-e")
                }
            )
        ),
        case!(
            "simd2-sidechain-submix",
            MissingEntityReference,
            "$.tracks[0].simd2.effects[0].sidechain.source.submix_id",
            |s| set_sidechain(
                s,
                RackName::Simd2,
                RouteSource::SubmixOutput {
                    submix_id: id("missing-f")
                }
            )
        ),
        case!(
            "automation-entity",
            MissingEntityReference,
            "$.automation[0].target.entity_id",
            |s| s.automation[0].target.entity_id = id("missing-track")
        ),
        case!(
            "automation-effect",
            MissingEntityReference,
            "$.automation[0].target.effect_id",
            |s| s.automation[0].target.effect_id = id("missing-effect")
        ),
        case!(
            "automation-param",
            MissingEntityReference,
            "$.automation[0].target.parameter_id",
            |s| s.automation[0].target.parameter_id = 999
        ),
        case!(
            "automation-channel",
            MissingEntityReference,
            "$.automation[0].target.parameter_id",
            |s| s.automation[0].target.channel = ParameterChannel::Left
        ),
        case!(
            "duplicate-automation",
            DuplicateId,
            "$.automation[1].id",
            |s| s.automation.push(s.automation[0].clone())
        ),
        case!(
            "empty-automation",
            NumericOutOfSchemaRange,
            "$.automation[0].segments",
            |s| s.automation[0].segments.clear()
        ),
        case!(
            "automation-equal-end",
            AutomationInvalidRange,
            "$.automation[0].segments[0].end_sample",
            |s| s.automation[0].segments[0].end_sample = 0
        ),
        case!(
            "automation-reversed",
            AutomationInvalidRange,
            "$.automation[0].segments[0].end_sample",
            |s| {
                s.automation[0].segments[0].start_sample = 2;
                s.automation[0].segments[0].end_sample = 1
            }
        ),
        case!(
            "automation-overlap",
            AutomationSegmentOverlap,
            "$.automation[0].segments[1].start_sample",
            |s| {
                let mut p = s.automation[0].segments[0].clone();
                p.start_sample = 479;
                p.end_sample = 960;
                s.automation[0].segments.push(p)
            }
        ),
        case!(
            "automation-order",
            AutomationOutOfOrder,
            "$.automation[0].segments[1].start_sample",
            |s| {
                s.automation[0].segments[0].start_sample = 500;
                s.automation[0].segments[0].end_sample = 600;
                let mut p = s.automation[0].segments[0].clone();
                p.start_sample = 100;
                p.end_sample = 200;
                s.automation[0].segments.push(p)
            }
        ),
        case!(
            "exponential-zero-start",
            AutomationInvalidRange,
            "$.automation[0].segments[0].start_value",
            |s| {
                let p = &mut s.automation[0].segments[0];
                p.shape = AutomationShape::Exponential;
                p.start_value = 0.0;
                p.end_value = 1.0
            }
        ),
        case!(
            "exponential-zero-end",
            AutomationInvalidRange,
            "$.automation[0].segments[0].end_value",
            |s| {
                let p = &mut s.automation[0].segments[0];
                p.shape = AutomationShape::Exponential;
                p.start_value = 1.0;
                p.end_value = 0.0
            }
        ),
        case!(
            "exponential-negative-start",
            AutomationInvalidRange,
            "$.automation[0].segments[0].start_value",
            |s| {
                let p = &mut s.automation[0].segments[0];
                p.shape = AutomationShape::Exponential;
                p.start_value = -1.0;
                p.end_value = 1.0
            }
        ),
        case!(
            "exponential-negative-end",
            AutomationInvalidRange,
            "$.automation[0].segments[0].end_value",
            |s| {
                let p = &mut s.automation[0].segments[0];
                p.shape = AutomationShape::Exponential;
                p.start_value = 1.0;
                p.end_value = -1.0
            }
        ),
        case!(
            "automation-hz-start",
            NumericOutOfSchemaRange,
            "$.automation[0].segments[0].start_value",
            |s| {
                let p = &mut s.automation[0].segments[0];
                p.unit = ParameterUnit::Hz;
                p.start_value = -1.0
            }
        ),
        case!(
            "automation-ms-end",
            NumericOutOfSchemaRange,
            "$.automation[0].segments[0].end_value",
            |s| {
                let p = &mut s.automation[0].segments[0];
                p.unit = ParameterUnit::Milliseconds;
                p.end_value = -1.0
            }
        ),
        case!(
            "automation-samples-start",
            NumericOutOfSchemaRange,
            "$.automation[0].segments[0].start_value",
            |s| {
                let p = &mut s.automation[0].segments[0];
                p.unit = ParameterUnit::Samples;
                p.start_value = 1.5
            }
        ),
        case!(
            "automation-samples-end",
            NumericOutOfSchemaRange,
            "$.automation[0].segments[0].end_value",
            |s| {
                let p = &mut s.automation[0].segments[0];
                p.unit = ParameterUnit::Samples;
                p.end_value = 2.5
            }
        ),
    ];
    assert!(
        cases.len() >= 75,
        "complete compile-expressible mutation inventory"
    );
    for case in &cases {
        let mut model = parse_session_toml(CANONICAL).expect("fixture parses");
        (case.typed)(&mut model);
        let text = unvalidated_toml(&model);
        let parsed = parse_session_toml(&text).expect_err(case.name);
        let span = target(&parsed, case).span.expect("parse span");
        assert!(
            span.byte_start < span.byte_end && span.byte_end <= text.len(),
            "{} span",
            case.name
        );
        assert!(span.line > 0 && span.column > 0, "{} line", case.name);
        let canonical = canonical_session_toml(&model).expect_err(case.name);
        let compiled = compile_session(&model, caps()).expect_err(case.name);
        assert!(canonical.diagnostics().iter().all(|d| d.span.is_none()));
        assert!(compiled.diagnostics().iter().all(|d| d.span.is_none()));
        assert_eq!(
            (
                target(&canonical, case).code,
                target(&canonical, case).path.to_string()
            ),
            (case.code, case.path.to_owned())
        );
        assert_eq!(
            (
                target(&compiled, case).code,
                target(&compiled, case).path.to_string()
            ),
            (case.code, case.path.to_owned())
        );
    }
    let wrong_type = CANONICAL.replacen("revision = 7", "revision = \"7\"", 1);
    let errors = parse_session_toml(&wrong_type).expect_err("wrong type");
    let wrong = Case {
        name: "wrong-type",
        code: DiagnosticCode::WrongType,
        path: "$.revision",
        typed: |_| {},
    };
    let span = target(&errors, &wrong).span.expect("wrong type span");
    assert_eq!(&wrong_type[span.byte_start..span.byte_end], "\"7\"");
}

struct Writer {
    output: String,
    depth: usize,
    first: bool,
}
impl Writer {
    fn field(&mut self, key: FieldKey) {
        if self.depth != 0 && !self.first {
            self.output.push_str(", ");
        }
        self.output.push_str(key.name);
        self.output.push_str(" = ");
        self.first = false;
    }
    fn scalar_end(&mut self) {
        if self.depth == 0 {
            self.output.push('\n');
        }
    }
}
impl ModelVisitor for Writer {
    type Error = Infallible;
    fn record_begin(&mut self, key: Option<FieldKey>, _: u32) -> Result<(), Self::Error> {
        if let Some(key) = key {
            self.field(key);
            self.output.push_str("{ ");
        } else if self.depth != 0 {
            if self.depth == 1 {
                self.output.push_str("  ");
            } else if !self.first {
                self.output.push_str(", ");
            }
            self.output.push_str("{ ");
        } else {
            return Ok(());
        }
        self.depth += 1;
        self.first = true;
        Ok(())
    }
    fn record_end(&mut self) -> Result<(), Self::Error> {
        if self.depth == 0 {
            return Ok(());
        }
        self.depth -= 1;
        self.output.push_str(" }");
        if self.depth == 0 {
            self.output.push('\n');
        } else if self.depth == 1 {
            self.output.push_str(",\n");
        }
        self.first = false;
        Ok(())
    }
    fn array_begin(&mut self, key: FieldKey, _: usize) -> Result<(), Self::Error> {
        self.field(key);
        self.output.push('[');
        if self.depth == 0 {
            self.output.push('\n');
        }
        self.depth += 1;
        self.first = true;
        Ok(())
    }
    fn array_end(&mut self) -> Result<(), Self::Error> {
        self.depth -= 1;
        self.output.push(']');
        if self.depth == 0 {
            self.output.push('\n');
        }
        self.first = false;
        Ok(())
    }
    fn wire_tag(&mut self, _: Token) -> Result<(), Self::Error> {
        Ok(())
    }
    fn bool(&mut self, key: FieldKey, value: bool) -> Result<(), Self::Error> {
        self.field(key);
        self.output.push_str(if value { "true" } else { "false" });
        self.scalar_end();
        Ok(())
    }
    fn u8(&mut self, key: FieldKey, value: u8) -> Result<(), Self::Error> {
        self.u64(key, u64::from(value))
    }
    fn u32(&mut self, key: FieldKey, value: u32) -> Result<(), Self::Error> {
        self.u64(key, u64::from(value))
    }
    fn u64(&mut self, key: FieldKey, value: u64) -> Result<(), Self::Error> {
        self.field(key);
        let _ = write!(self.output, "{value}");
        self.scalar_end();
        Ok(())
    }
    fn source_bit_depth(
        &mut self,
        key: FieldKey,
        value: session::SourceBitDepth,
    ) -> Result<(), Self::Error> {
        self.field(key);
        match value {
            session::SourceBitDepth::Pcm16 => self.output.push_str("16"),
            session::SourceBitDepth::Pcm24 => self.output.push_str("24"),
            session::SourceBitDepth::Float32 => {
                write_quoted(&mut self.output, "32f");
            }
        }
        self.scalar_end();
        Ok(())
    }
    fn f32(&mut self, key: FieldKey, value: f32) -> Result<(), Self::Error> {
        self.field(key);
        if value.is_nan() {
            self.output.push_str("nan");
        } else if value == f32::INFINITY {
            self.output.push_str("inf");
        } else if value == f32::NEG_INFINITY {
            self.output.push_str("-inf");
        } else {
            let _ = write!(self.output, "{value}");
        }
        self.scalar_end();
        Ok(())
    }
    fn id(&mut self, key: FieldKey, value: &StableId) -> Result<(), Self::Error> {
        self.text(key, value.as_str())
    }
    fn text(&mut self, key: FieldKey, value: &str) -> Result<(), Self::Error> {
        self.field(key);
        write_quoted(&mut self.output, value);
        self.scalar_end();
        Ok(())
    }
    fn token(&mut self, key: FieldKey, value: Token) -> Result<(), Self::Error> {
        self.text(key, value.text)
    }
}
fn write_quoted(output: &mut String, value: &str) {
    output.push('"');
    for ch in value.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\t' => output.push_str("\\t"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            ch => output.push(ch),
        }
    }
    output.push('"');
}
fn unvalidated_toml(model: &SessionToml) -> String {
    let mut writer = Writer {
        output: String::new(),
        depth: 0,
        first: true,
    };
    match model.visit(WalkOrder::Declared, &mut writer) {
        Ok(()) => writer.output,
        Err(error) => match error {},
    }
}
