//! E5: every text-expressible validator mutation has text/typed entry-point parity.

use session::{
    AutomationSegment, AutomationShape, CompileCaps, Console, ConsoleEntry, ConsoleSlot,
    DiagnosticCode, DiagnosticSet, EffectIdentity, MatrixOrPan, Output, ParameterChannel,
    ParameterUnit, RouteDestination, RouteSource, SendTap, SessionModel, Sidechain,
    SidechainDeclaration, StableId, canonical_session_json, compile_session, parse_session_json,
};

const CANONICAL: &str = include_str!("../../../fixtures/session/v1/canonical.json");
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
    typed: fn(&mut SessionModel),
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
/// Set a routed sidechain on insert `position`: `0` rewrites the fixture's one insert, `1` appends
/// a second insert so the path's index is exercised past the first element.
fn set_sidechain(model: &mut SessionModel, position: usize, source: RouteSource) {
    let mut effect = model.tracks[0].inserts.effects[0].clone();
    effect.sidechain = SidechainDeclaration::Routed(Sidechain {
        source,
        port_id: id("detector-in"),
    });
    if position == 0 {
        model.tracks[0].inserts.effects[0] = effect;
    } else {
        effect.id = id("keyed");
        model.tracks[0].inserts.effects.push(effect);
    }
}

/// Install a console with one slot per section and the one fixture track's entries for them.
fn with_console(model: &mut SessionModel) {
    let slot = |name: &str, effect_id: &str| ConsoleSlot {
        slot: id(name),
        identity: EffectIdentity::Native {
            effect_id: id(effect_id),
        },
        quality: session::EffectQuality::Normal,
        link_mode: session::LinkMode::DualMono,
    };
    model.console = Console {
        pre_insert: vec![slot("desk-eq", "miso.parametric-eq")],
        post_insert: vec![slot("desk-limit", "miso.true-peak-limiter")],
    };
    let entry = |name: &str| ConsoleEntry {
        slot: id(name),
        bypass: false,
        params: model.tracks[0].inserts.effects[0].params.clone(),
    };
    model.tracks[0].console = vec![entry("desk-eq"), entry("desk-limit")];
}
/// Replace the fixture's automation with two fader (`builtins` 5) entries on its track, `fader-a`
/// then `fader-b`, on the given lanes.
fn two_fader_rides(model: &mut SessionModel, first: ParameterChannel, second: ParameterChannel) {
    let ride = |name: &str, lane| {
        let mut entry = model.automation[0].clone();
        entry.id = id(name);
        entry.target.section = session::Section::Builtins;
        entry.target.control = 5;
        entry.target.lane = lane;
        entry
    };
    let rides = vec![ride("fader-a", first), ride("fader-b", second)];
    model.automation = rides;
}

/// The fixture's one automation entry gets two segments that start at the given samples, a `step`
/// from 0 to 1 on `[0, 960)` and then a `linear` one from `second_start` that begins at 0.5, so the
/// discontinuities are at 960 and `second_start`.
fn step_then_ramp(model: &mut SessionModel, second_start: u64) {
    let step = AutomationSegment {
        shape: AutomationShape::Step,
        start_sample: 0,
        end_sample: 960,
        start_value: 0.0,
        end_value: 1.0,
        unit: ParameterUnit::Db,
    };
    let ramp = AutomationSegment {
        shape: AutomationShape::Linear,
        start_sample: second_start,
        end_sample: second_start + 480,
        start_value: 0.5,
        end_value: 0.0,
        unit: ParameterUnit::Db,
    };
    model.automation[0].segments = vec![step, ramp];
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
        case!(
            "empty-cid",
            NumericOutOfSchemaRange,
            "$.tracks[0].inserts.effects[0].identity.cid",
            |s| s.tracks[0].inserts.effects[0].identity =
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
            "$.tracks[0].inserts.effects[0].params[0].value",
            |s| s.tracks[0].inserts.effects[0].params[0].value = f32::NAN
        ),
        case!(
            "param-negative",
            NumericOutOfSchemaRange,
            "$.tracks[0].inserts.effects[0].params[0].value",
            |s| {
                let p = &mut s.tracks[0].inserts.effects[0].params[0];
                p.unit = ParameterUnit::Hz;
                p.value = -1.0
            }
        ),
        case!(
            "param-fraction",
            NumericOutOfSchemaRange,
            "$.tracks[0].inserts.effects[0].params[0].value",
            |s| {
                let p = &mut s.tracks[0].inserts.effects[0].params[0];
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
            |s| s.sources[0].content = "blake3:abc".to_owned()
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
            "source-channels-zero",
            SourceChannelsUnsupported,
            "$.sources[0].channels",
            |s| s.sources[0].channels = 0
        ),
        case!(
            "source-channels-three",
            SourceChannelsUnsupported,
            "$.sources[0].channels",
            |s| s.sources[0].channels = 3
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
            |s| s.routes[0].source = RouteSource::Submix {
                submix_id: id("missing-mix-a"),
                tap: SendTap::PostPan,
            }
        ),
        case!(
            "route-submix-b",
            MissingEntityReference,
            "$.routes[1].source.submix_id",
            |s| {
                let mut r = s.routes[0].clone();
                r.id = id("route-b");
                r.source = RouteSource::Submix {
                    submix_id: id("missing-mix-b"),
                    tap: SendTap::PostPan,
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
            "insert-sidechain-track",
            MissingEntityReference,
            "$.tracks[0].inserts.effects[0].sidechain.source.track_id",
            |s| set_sidechain(
                s,
                0,
                RouteSource::Track {
                    track_id: id("missing-a"),
                    tap: SendTap::InsertReturn
                }
            )
        ),
        case!(
            "second-insert-sidechain-track",
            MissingEntityReference,
            "$.tracks[0].inserts.effects[1].sidechain.source.track_id",
            |s| set_sidechain(
                s,
                1,
                RouteSource::Track {
                    track_id: id("missing-b"),
                    tap: SendTap::PostPan
                }
            )
        ),
        case!(
            "insert-sidechain-submix",
            MissingEntityReference,
            "$.tracks[0].inserts.effects[0].sidechain.source.submix_id",
            |s| set_sidechain(
                s,
                0,
                RouteSource::Submix {
                    submix_id: id("missing-d"),
                    tap: SendTap::PostPan,
                }
            )
        ),
        case!(
            "second-insert-sidechain-submix",
            MissingEntityReference,
            "$.tracks[0].inserts.effects[1].sidechain.source.submix_id",
            |s| set_sidechain(
                s,
                1,
                RouteSource::Submix {
                    submix_id: id("missing-e"),
                    tap: SendTap::PostPan,
                }
            )
        ),
        // Decision 12's console refusals, each at its own path.
        case!(
            "console-slot-repeated-across-sections",
            DuplicateId,
            "$.console.post_insert[0].slot",
            |s| {
                with_console(s);
                s.console.post_insert[0].slot = id("desk-eq");
            }
        ),
        case!(
            "console-slot-third-party",
            ConsoleSlotNotNative,
            "$.console.pre_insert[0].identity",
            |s| {
                with_console(s);
                s.console.pre_insert[0].identity = EffectIdentity::ThirdPartyCid {
                    cid: "bafyopaque".to_owned(),
                };
            }
        ),
        case!(
            "console-entry-unknown-slot",
            MissingEntityReference,
            "$.tracks[0].console[1].slot",
            |s| {
                with_console(s);
                s.tracks[0].console[1].slot = id("desk-undeclared");
            }
        ),
        case!(
            "console-entry-duplicate-slot",
            DuplicateId,
            "$.tracks[0].console[1].slot",
            |s| {
                with_console(s);
                s.tracks[0].console[1].slot = id("desk-eq");
            }
        ),
        case!(
            "console-entry-misordered",
            ConsoleEntryOrder,
            "$.tracks[0].console[0].slot",
            |s| {
                with_console(s);
                s.tracks[0].console.swap(0, 1);
            }
        ),
        case!(
            "console-entry-missing",
            ConsoleEntryMissing,
            "$.tracks[0].console",
            |s| {
                with_console(s);
                s.tracks[0].console.pop();
            }
        ),
        case!(
            "console-param-finite",
            NumericNonFinite,
            "$.tracks[0].console[1].params[0].value",
            |s| {
                with_console(s);
                s.tracks[0].console[1].params[0].value = f32::NAN;
            }
        ),
        case!(
            "console-param-duplicate",
            DuplicateId,
            "$.tracks[0].console[0].params[1].parameter_id",
            |s| {
                with_console(s);
                let repeated = s.tracks[0].console[0].params[0].clone();
                s.tracks[0].console[0].params.push(repeated);
            }
        ),
        case!(
            "automation-console-slot",
            MissingEntityReference,
            "$.automation[0].target.slot_id",
            |s| {
                with_console(s);
                s.automation[0].target.section = session::Section::Console(id("eq"));
            }
        ),
        case!(
            "automation-entity",
            MissingEntityReference,
            "$.automation[0].target.owner_id",
            |s| s.automation[0].target.owner = session::Owner::Strip(id("missing-track"))
        ),
        case!(
            "automation-effect",
            MissingEntityReference,
            "$.automation[0].target.slot_id",
            |s| s.automation[0].target.section = session::Section::Inserts(id("missing-effect"))
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
            "lane-both-both",
            AutomationDuplicateLane,
            "$.automation[1].target.lane",
            |s| two_fader_rides(s, ParameterChannel::Both, ParameterChannel::Both)
        ),
        case!(
            "lane-both-left",
            AutomationDuplicateLane,
            "$.automation[1].target.lane",
            |s| two_fader_rides(s, ParameterChannel::Both, ParameterChannel::Left)
        ),
        case!(
            "lane-left-left",
            AutomationDuplicateLane,
            "$.automation[1].target.lane",
            |s| two_fader_rides(s, ParameterChannel::Left, ParameterChannel::Left)
        ),
        case!(
            "lane-insert-copy",
            AutomationDuplicateLane,
            "$.automation[1].target.lane",
            |s| {
                let mut copy = s.automation[0].clone();
                copy.id = id("eq-gain-copy");
                s.automation.push(copy)
            }
        ),
        case!(
            "discontinuity-spacing",
            AutomationDiscontinuitySpacing,
            "$.automation[0].segments[1].start_sample",
            |s| step_then_ramp(s, 1023)
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
        case!(
            "automation-end-at-time-limit",
            NumericOutOfSchemaRange,
            "$.automation[0].segments[0].end_sample",
            |s| {
                let p = &mut s.automation[0].segments[0];
                p.start_sample = 0;
                p.end_sample = automation::TIME_LIMIT
            }
        ),
        case!(
            "automation-start-at-time-limit",
            NumericOutOfSchemaRange,
            "$.automation[0].segments[0].start_sample",
            |s| {
                let p = &mut s.automation[0].segments[0];
                p.start_sample = automation::TIME_LIMIT;
                p.end_sample = automation::TIME_LIMIT + 1
            }
        ),
    ];
    assert!(
        cases.len() >= 70,
        "complete compile-expressible mutation inventory"
    );
    for case in &cases {
        let mut model = parse_session_json(CANONICAL).expect("fixture parses");
        (case.typed)(&mut model);
        let canonical = canonical_session_json(&model).expect_err(case.name);
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
    let wrong_type = CANONICAL.replacen("\"revision\": \"7\"", "\"revision\": 7", 1);
    let errors = parse_session_json(&wrong_type).expect_err("wrong type");
    let wrong = Case {
        name: "wrong-type",
        code: DiagnosticCode::WrongType,
        path: "$.revision",
        typed: |_| {},
    };
    let span = target(&errors, &wrong).span.expect("wrong type span");
    assert_eq!(&wrong_type[span.byte_start..span.byte_end], "7");
}

/// The revision domain is `0..=MAX_SESSION_REVISION` through every entry point (#1503): text
/// parse, typed canonicalization and typed compilation each refuse exactly one
/// `NumericOutOfSchemaRange` at `$.revision`, and the ceiling itself is accepted and round-trips.
#[test]
fn revision_above_the_plan_gate_ceiling_is_refused_by_every_entry_point() {
    let at_ceiling = CANONICAL.replacen(
        "\"revision\": \"7\"",
        &format!("\"revision\": \"{}\"", session::MAX_SESSION_REVISION),
        1,
    );
    let model = parse_session_json(&at_ceiling).expect("the ceiling parses");
    assert_eq!(model.revision, session::MAX_SESSION_REVISION);
    assert_eq!(
        canonical_session_json(&model).expect("the ceiling writes"),
        at_ceiling
    );
    compile_session(&model, caps()).expect("the ceiling compiles");

    for revision in [session::MAX_SESSION_REVISION + 1, u64::MAX] {
        let source = CANONICAL.replacen(
            "\"revision\": \"7\"",
            &format!("\"revision\": \"{revision}\""),
            1,
        );
        let mut model = parse_session_json(CANONICAL).expect("fixture parses");
        model.revision = revision;
        let sets = [
            parse_session_json(&source).expect_err("text parse"),
            canonical_session_json(&model).expect_err("typed canonical"),
            compile_session(&model, caps()).expect_err("typed compile"),
        ];
        for set in &sets {
            let found: Vec<_> = set
                .diagnostics()
                .iter()
                .map(|d| (d.code, d.path.to_string()))
                .collect();
            assert_eq!(
                found,
                vec![(
                    DiagnosticCode::NumericOutOfSchemaRange,
                    "$.revision".to_owned()
                )],
                "revision {revision}"
            );
        }
        let span = sets[0].diagnostics()[0].span.expect("parse span");
        assert_eq!(
            &source[span.byte_start..span.byte_end],
            format!("\"{revision}\"")
        );
    }
}

/// The session validator and `automation::ProgramBuilder` agree on which segment samples are
/// allowed (#1646b departure 2): a segment the validator accepts never reaches
/// `BuildError::Segment` at preparation, and one the builder refuses for its time is refused by
/// the validator. Red if either side moves its bound (`2^53`) alone.
#[test]
fn the_validator_and_the_program_builder_agree_on_the_segment_time_bound() {
    use automation::{
        BuildError, CellDescription, Completion, JumpLength, ProgramBuilder, Segment, Shape,
        TIME_LIMIT,
    };
    assert_eq!(TIME_LIMIT, 1 << 53, "the bound is exact in f64");
    let edges = [
        (0, 1, true),
        (TIME_LIMIT - 2, TIME_LIMIT - 1, true),
        (0, TIME_LIMIT - 1, true),
        (0, TIME_LIMIT, false),
        (TIME_LIMIT - 1, TIME_LIMIT, false),
        (TIME_LIMIT, TIME_LIMIT + 1, false),
        (0, u64::MAX, false),
        (u64::MAX - 1, u64::MAX, false),
    ];
    for (start, end, accepted) in edges {
        let mut model = parse_session_json(CANONICAL).expect("fixture parses");
        let segment = &mut model.automation[0].segments[0];
        segment.shape = AutomationShape::Step;
        segment.start_sample = start;
        segment.end_sample = end;
        let validator_accepts = canonical_session_json(&model).is_ok();
        let segments = [Segment {
            start,
            end,
            start_value: 0.0,
            end_value: 0.0,
            shape: Shape::Step,
        }];
        let builder_accepts = !matches!(
            ProgramBuilder::build(&[CellDescription {
                segments: &segments,
                grid_ramp: 64,
                grid: 64,
                jump: JumpLength::Block,
                completion: Completion::End,
                no_restart: false,
            }]),
            Err(BuildError::Segment { .. })
        );
        assert_eq!(validator_accepts, accepted, "validator at {start}..{end}");
        assert_eq!(builder_accepts, accepted, "builder at {start}..{end}");
    }
}
