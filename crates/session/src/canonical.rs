//! Canonical JSON writer over the shared emit-side model walk.

use core::convert::Infallible;

use crate::{
    FieldKey, ModelVisitor, SessionModel, Token, VisitModel, WalkOrder,
    validate::validate_session,
    value::{Spelling, spell_f32},
};

/// Produce canonical Session V1 JSON as UTF-8 text with one final newline.
pub fn canonical_session_json(session: &SessionModel) -> Result<String, crate::DiagnosticSet> {
    validate_session(session)?;
    Ok(write_canonical(session))
}

pub(crate) fn write_canonical(session: &SessionModel) -> String {
    let mut writer = JsonWriter {
        output: String::new(),
        containers: Vec::new(),
    };
    match session.visit(WalkOrder::Canonical, &mut writer) {
        Ok(()) => {
            writer.output.push('\n');
            // The writer grows its `String` by doubling, so it ends with up to half its allocation
            // spare (16,056 of 32,768 bytes on the nine-track EQ fixture). A compiled session
            // keeps this snapshot for its whole life and the model estimate charges its length,
            // so give the spare back here, on the control plane, with the one reallocation it
            // costs (#1060).
            writer.output.shrink_to_fit();
            writer.output
        }
        Err(error) => match error {},
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Kind {
    Object,
    Array,
}
#[derive(Clone, Copy)]
struct Container {
    kind: Kind,
    first: bool,
    empty: bool,
}
/// Where the canonical writer puts its text: a `String` (the document) or a [`ByteCount`] (the
/// length of a part of it, with no buffer).
trait Sink: core::fmt::Write {
    fn push_str(&mut self, text: &str) {
        let _ = self.write_str(text);
    }
    fn push(&mut self, character: char) {
        let _ = self.write_char(character);
    }
    /// One canonical `f32` spelling.
    fn f32(&mut self, value: f32);
}

impl Sink for String {
    fn f32(&mut self, value: f32) {
        spell_f32(self, value);
    }
}

/// A sink that keeps the number of bytes written and nothing else.
#[derive(Default)]
struct ByteCount(usize);

impl core::fmt::Write for ByteCount {
    fn write_str(&mut self, text: &str) -> core::fmt::Result {
        self.0 += text.len();
        Ok(())
    }
    fn write_char(&mut self, character: char) -> core::fmt::Result {
        self.0 += character.len_utf8();
        Ok(())
    }
}

impl Sink for ByteCount {
    fn f32(&mut self, value: f32) {
        // A number is spelled in a fixed buffer on the stack, then counted.
        let mut spelling = Spelling::new();
        spell_f32(&mut spelling, value);
        self.0 += spelling.len();
    }
}

struct JsonWriter<S: Sink> {
    output: S,
    containers: Vec<Container>,
}

impl<S: Sink> JsonWriter<S> {
    fn indent(&mut self, depth: usize) {
        for _ in 0..depth * 2 {
            self.output.push(' ');
        }
    }
    fn entry_prefix(&mut self) {
        let depth = self.containers.len();
        let container = self.containers.last_mut().expect("value has a parent");
        if !container.first {
            self.output.push(',');
        }
        self.output.push('\n');
        container.first = false;
        self.indent(depth);
    }
    fn field(&mut self, key: FieldKey) {
        debug_assert_eq!(self.containers.last().map(|c| c.kind), Some(Kind::Object));
        self.entry_prefix();
        write_quoted(&mut self.output, key.name);
        self.output.push_str(": ");
    }
    fn array_item(&mut self) {
        debug_assert_eq!(self.containers.last().map(|c| c.kind), Some(Kind::Array));
        self.entry_prefix();
    }
    fn close(&mut self, expected: Kind, delimiter: char) {
        let container = self.containers.pop().expect("balanced model walk");
        debug_assert_eq!(container.kind, expected);
        if !container.empty {
            self.output.push('\n');
            self.indent(self.containers.len());
        }
        self.output.push(delimiter);
    }
}

impl<S: Sink> ModelVisitor for JsonWriter<S> {
    type Error = Infallible;
    fn record_begin(&mut self, key: Option<FieldKey>, _fields: u32) -> Result<(), Self::Error> {
        match key {
            Some(key) => self.field(key),
            None if !self.containers.is_empty() => self.array_item(),
            None => {}
        }
        self.output.push('{');
        self.containers.push(Container {
            kind: Kind::Object,
            first: true,
            empty: false,
        });
        Ok(())
    }
    fn record_end(&mut self) -> Result<(), Self::Error> {
        self.close(Kind::Object, '}');
        Ok(())
    }
    fn array_begin(&mut self, key: FieldKey, length: usize) -> Result<(), Self::Error> {
        self.field(key);
        self.output.push('[');
        self.containers.push(Container {
            kind: Kind::Array,
            first: true,
            empty: length == 0,
        });
        Ok(())
    }
    fn array_end(&mut self) -> Result<(), Self::Error> {
        self.close(Kind::Array, ']');
        Ok(())
    }
    fn wire_tag(&mut self, _: Token) -> Result<(), Self::Error> {
        Ok(())
    }
    fn bool(&mut self, key: FieldKey, value: bool) -> Result<(), Self::Error> {
        self.field(key);
        self.output.push_str(if value { "true" } else { "false" });
        Ok(())
    }
    fn u8(&mut self, key: FieldKey, value: u8) -> Result<(), Self::Error> {
        self.field(key);
        let _ = write!(self.output, "{value}");
        Ok(())
    }
    fn u32(&mut self, key: FieldKey, value: u32) -> Result<(), Self::Error> {
        self.field(key);
        let _ = write!(self.output, "{value}");
        Ok(())
    }
    fn u64(&mut self, key: FieldKey, value: u64) -> Result<(), Self::Error> {
        self.field(key);
        let _ = write!(self.output, "\"{value}\"");
        Ok(())
    }
    fn source_bit_depth(
        &mut self,
        key: FieldKey,
        value: crate::SourceBitDepth,
    ) -> Result<(), Self::Error> {
        self.field(key);
        match value {
            crate::SourceBitDepth::Pcm16 => self.output.push_str("16"),
            crate::SourceBitDepth::Pcm24 => self.output.push_str("24"),
            crate::SourceBitDepth::Float32 => write_quoted(&mut self.output, "32f"),
        }
        Ok(())
    }
    fn f32(&mut self, key: FieldKey, value: f32) -> Result<(), Self::Error> {
        self.field(key);
        self.output.f32(value);
        Ok(())
    }
    fn id(&mut self, key: FieldKey, value: &crate::StableId) -> Result<(), Self::Error> {
        self.text(key, value.as_str())
    }
    fn id_item(&mut self, value: &crate::StableId) -> Result<(), Self::Error> {
        self.array_item();
        write_quoted(&mut self.output, value.as_str());
        Ok(())
    }
    fn text(&mut self, key: FieldKey, value: &str) -> Result<(), Self::Error> {
        self.field(key);
        write_quoted(&mut self.output, value);
        Ok(())
    }
    fn token(&mut self, key: FieldKey, value: Token) -> Result<(), Self::Error> {
        self.text(key, value.text)
    }
}

fn write_quoted<S: Sink>(output: &mut S, value: &str) {
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\u{08}' => output.push_str("\\b"),
            '\t' => output.push_str("\\t"),
            '\n' => output.push_str("\\n"),
            '\u{0C}' => output.push_str("\\f"),
            '\r' => output.push_str("\\r"),
            character if character.is_control() => {
                let _ = write!(output, "\\u{:04X}", u32::from(character));
            }
            character => output.push(character),
        }
    }
    output.push('"');
}

/// The bytes one owner (a track, a submix, a VCA or a route) takes in the canonical text of a
/// session, at the depth of an item of the root's arrays: the walk of [`write_canonical`] over that
/// value alone, indented as in the document. The bytes of a document are the sum of its owners'
/// bytes and of the rest, so a value change moves the document's length by the change of its
/// owner's (#1613).
pub(crate) fn owner_bytes<T: VisitModel>(owner: &T) -> usize {
    let mut writer = JsonWriter {
        output: ByteCount::default(),
        containers: vec![
            Container {
                kind: Kind::Object,
                first: false,
                empty: false,
            },
            Container {
                kind: Kind::Array,
                first: true,
                empty: false,
            },
        ],
    };
    match owner.visit(WalkOrder::Canonical, &mut writer) {
        Ok(()) => writer.output.0,
        Err(error) => match error {},
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use serde_json::json;

    use super::{ByteCount, Sink, write_quoted};
    use crate::value::write_f32;
    use crate::{
        Automation, AutomationOffset, AutomationSegment, AutomationShape, ChannelBuiltins,
        ChannelMatrix, Console, ConsoleEntry, ConsoleSlot, ControlAddress, DualMonoBuiltins,
        DualMonoFader, Effect, EffectIdentity, EffectParam, EffectQuality, LinkMode, MatrixOrPan,
        Owner, ParameterChannel, ParameterUnit, Rack, Route, RouteDestination, RouteSource,
        Section, SendTap, Sidechain, SidechainDeclaration, StableId, canonical_session_json,
        parse_session_json,
    };
    #[test]
    fn canonical_string_escaping_is_frozen() {
        let mut output = String::new();
        write_quoted(
            &mut output,
            "\"\\\u{8}\t\n\u{c}\r\u{0}\u{7f}\u{80}/\u{2028}\u{2029}\u{1f642}",
        );
        assert_eq!(
            output,
            "\"\\\"\\\\\\b\\t\\n\\f\\r\\u0000\\u007F\\u0080/\u{2028}\u{2029}\u{1f642}\""
        );
    }

    /// The counting writer's `f32` length is the `String` writer's length, for the extremes (the
    /// smallest and largest of each sign, which spell longest) and a sweep of every exponent.
    #[test]
    fn the_counting_sink_counts_the_bytes_of_the_string_sink() {
        let mut bits = vec![
            0,
            0x8000_0000,
            1,
            0x8000_0001,
            0x007f_ffff,
            0x0080_0000,
            0x7f7f_ffff,
            0xff7f_ffff,
            0x3f80_0000,
        ];
        for exponent in 0..255_u32 {
            for mantissa in [0, 1, 0x2a_aaaa, 0x7f_ffff] {
                bits.push(exponent << 23 | mantissa);
                bits.push(0x8000_0000 | exponent << 23 | mantissa);
            }
        }
        for pattern in bits {
            let value = f32::from_bits(pattern);
            let mut text = String::new();
            Sink::f32(&mut text, value);
            let mut counted = ByteCount::default();
            counted.f32(value);
            assert_eq!(counted.0, text.len(), "{pattern:#010x}: {text}");
        }
    }

    fn corpus_path(relative: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(relative)
    }

    fn f32_case(id: &str, bits: u32) -> serde_json::Value {
        let value = f32::from_bits(bits);
        assert!(value.is_finite(), "corpus case {id} is finite");
        let mut canonical = String::new();
        write_f32(&mut canonical, value);
        json!({"id": id, "bits": format!("{bits:08x}"), "canonical": canonical})
    }

    fn string_case(id: &str, value: &str) -> serde_json::Value {
        let mut canonical = String::new();
        write_quoted(&mut canonical, value);
        json!({"id": id, "value": value, "canonical": canonical})
    }

    fn id(value: &str) -> StableId {
        StableId::parse(value).expect("valid corpus ID")
    }

    fn full_surface_model() -> crate::SessionModel {
        let source = fs::read_to_string(corpus_path("fixtures/session/v1/canonical.json"))
            .expect("read representative session");
        let mut model = parse_session_json(&source).expect("parse representative session");
        // A non-trivial submix strip (#1199): every input-section field off its identity, its
        // entries for both console slots declared below (#1202; one live with parameters out of
        // canonical order, one bypassed), one native insert with parameters out of canonical
        // order, a muted lane and a pan.
        model.submixes.push(crate::Submix {
            id: id("mix"),
            builtins: DualMonoBuiltins {
                left: ChannelBuiltins {
                    polarity_invert: true,
                    trim_db: -3.5,
                    hpf_hz: 40.0,
                    lpf_hz: 0.0,
                    delay_samples: 12,
                },
                right: ChannelBuiltins {
                    polarity_invert: false,
                    trim_db: 1.25,
                    hpf_hz: 0.0,
                    lpf_hz: 16_000.0,
                    delay_samples: 0,
                },
            },
            console: vec![
                ConsoleEntry {
                    slot: id("desk-eq"),
                    bypass: false,
                    params: vec![
                        EffectParam {
                            parameter_id: 4,
                            channel: ParameterChannel::Both,
                            unit: ParameterUnit::Db,
                            value: 1.5,
                        },
                        EffectParam {
                            parameter_id: 2,
                            channel: ParameterChannel::Right,
                            unit: ParameterUnit::Hz,
                            value: 180.0,
                        },
                    ],
                },
                ConsoleEntry {
                    slot: id("desk-limit"),
                    bypass: true,
                    params: Vec::new(),
                },
            ],
            inserts: Rack {
                effects: vec![Effect {
                    id: id("bus-eq"),
                    identity: EffectIdentity::Native {
                        effect_id: id("miso.parametric-eq"),
                    },
                    quality: EffectQuality::Normal,
                    bypass: false,
                    link_mode: LinkMode::DualMono,
                    params: vec![
                        EffectParam {
                            parameter_id: 3,
                            channel: ParameterChannel::Right,
                            unit: ParameterUnit::Db,
                            value: 2.0,
                        },
                        EffectParam {
                            parameter_id: 2,
                            channel: ParameterChannel::Both,
                            unit: ParameterUnit::Hz,
                            value: 250.0,
                        },
                    ],
                    sidechain: SidechainDeclaration::None,
                }],
            },
            fader: DualMonoFader {
                left_db: -6.0,
                right_db: -4.5,
                left_mute: false,
                right_mute: true,
            },
            matrix_or_pan: MatrixOrPan::Pan {
                left: -0.5,
                right: 0.25,
                smoothing_samples: 64,
            },
        });
        model.tracks[0].matrix_or_pan = MatrixOrPan::Matrix {
            ll: 1.25,
            lr: -0.25,
            rl: 0.5,
            rr: 0.75,
            smoothing_samples: 32,
        };
        // Decision 12's surface: a slot in each console section, the track's entries for them,
        // and a keyed third-party insert ahead of the fixture's own.
        model.console = Console {
            pre_insert: vec![ConsoleSlot {
                slot: id("desk-eq"),
                identity: EffectIdentity::Native {
                    effect_id: id("miso.parametric-eq"),
                },
                quality: EffectQuality::High,
                link_mode: LinkMode::DualMono,
            }],
            post_insert: vec![ConsoleSlot {
                slot: id("desk-limit"),
                identity: EffectIdentity::Native {
                    effect_id: id("miso.true-peak-limiter"),
                },
                quality: EffectQuality::Normal,
                link_mode: LinkMode::Maximum,
            }],
        };
        model.tracks[0].console = vec![
            ConsoleEntry {
                slot: id("desk-eq"),
                bypass: true,
                params: vec![EffectParam {
                    parameter_id: 2,
                    channel: ParameterChannel::Left,
                    unit: ParameterUnit::Db,
                    value: -1.5,
                }],
            },
            ConsoleEntry {
                slot: id("desk-limit"),
                bypass: false,
                params: Vec::new(),
            },
        ];
        model.tracks[0].inserts.effects.insert(
            0,
            Effect {
                id: id("external"),
                identity: EffectIdentity::ThirdPartyCid {
                    cid: "bafyopaque-v1-text".to_owned(),
                },
                quality: EffectQuality::High,
                bypass: true,
                link_mode: LinkMode::Maximum,
                params: Vec::new(),
                sidechain: SidechainDeclaration::Routed(Sidechain {
                    source: RouteSource::Track {
                        track_id: id("vocal"),
                        tap: SendTap::PostFader,
                    },
                    port_id: id("detector-in"),
                }),
            },
        );
        model.routes[0].destination = RouteDestination::SubmixInput {
            submix_id: id("mix"),
        };
        model.routes.push(Route {
            id: id("mix-to-main"),
            source: RouteSource::Submix {
                submix_id: id("mix"),
                tap: SendTap::PostPan,
            },
            destination: RouteDestination::OutputInput {
                output_id: id("main-out"),
            },
            channel_matrix: ChannelMatrix {
                ll: 1.0,
                lr: 0.0,
                rl: 0.0,
                rr: 1.0,
            },
            gain_db: 0.0,
            mute: false,
            follows_mute: false,
        });
        // A VCA forest (#1240), declared out of ID order with members out of ID order: `master`
        // nests `group` (a VCA) beside a track and the submix, `group` overlaps `master` on the
        // track, and `spare` has no members.
        let vca = |name: &str, left_db: f32, right_mute: bool, members: &[&str]| crate::Vca {
            id: id(name),
            fader: DualMonoFader {
                left_db,
                right_db: 2.25,
                left_mute: false,
                right_mute,
            },
            members: members.iter().map(|member| id(member)).collect(),
        };
        model.vcas = vec![
            vca("spare", 0.0, false, &[]),
            vca("master", -3.5, true, &["vocal", "mix", "group"]),
            vca("group", -144.0, false, &["vocal"]),
        ];
        // The explicit `control_smoothing` spelling (#1054): `-0.0` keeps its sign, 1000 is the bound.
        model.control_smoothing = Some(crate::ControlSmoothing {
            mute_ms: -0.0,
            fader_ms: 12.5,
            pan_ms: 1000.0,
        });
        // One stored automation per address section (#1566): `canonical.json` already automates an
        // insert; these add the builtins, console and own (route and VCA) targets, so the corpus
        // walks every arm of the target record. Declared out of ID order.
        let ride = |name: &str, target: ControlAddress| Automation {
            id: id(name),
            target,
            segments: vec![AutomationSegment {
                shape: AutomationShape::Linear,
                start_sample: 0,
                end_sample: 480,
                start_value: 0.0,
                end_value: -3.0,
                unit: ParameterUnit::Db,
            }],
            offset: AutomationOffset::default(),
        };
        model.automation.insert(
            0,
            ride(
                "vca-offset",
                ControlAddress {
                    owner: Owner::Vca(id("group")),
                    section: Section::Own,
                    control: 1,
                    lane: ParameterChannel::Both,
                },
            ),
        );
        model.automation.push(ride(
            "route-gain",
            ControlAddress {
                owner: Owner::Route(id("to-main")),
                section: Section::Own,
                control: 1,
                lane: ParameterChannel::Both,
            },
        ));
        // A hand move stored on the left fader ride (#1650 D1): the corpus walks a non-zero offset.
        model.automation.push(Automation {
            offset: AutomationOffset {
                left_db: 3.0,
                right_db: 0.0,
            },
            ..ride(
                "fader-ride",
                ControlAddress {
                    owner: Owner::Strip(id("vocal")),
                    section: Section::Builtins,
                    control: 5,
                    lane: ParameterChannel::Left,
                },
            )
        });
        model.automation.push(ride(
            "desk-gain",
            ControlAddress {
                owner: Owner::Strip(id("mix")),
                section: Section::Console(id("desk-eq")),
                control: 4,
                lane: ParameterChannel::Right,
            },
        ));
        model
    }

    fn full_surface_document() -> String {
        canonical_session_json(&full_surface_model()).expect("full tagged surface canonicalizes")
    }

    /// The full surface with the track's matrix and the submix's pan exchanged, so the corpus
    /// walks both arms of `matrix_or_pan` under both strip records (#1569).
    fn exchanged_strips_document() -> String {
        let mut model = full_surface_model();
        let matrix = std::mem::replace(
            &mut model.tracks[0].matrix_or_pan,
            MatrixOrPan::Pan {
                left: 0.75,
                right: -0.75,
                smoothing_samples: 8,
            },
        );
        model.submixes[0].matrix_or_pan = matrix;
        canonical_session_json(&model).expect("exchanged strips canonicalize")
    }

    #[test]
    fn canonical_writer_corpus_is_rust_generated_and_current() {
        let documents = [(
            "canonical-minimal",
            "fixtures/session/v1/canonical-minimal.json",
        )]
        .map(|(id, path)| {
            let source = fs::read_to_string(corpus_path(path)).expect("read corpus session");
            let model = parse_session_json(&source).expect("parse corpus session");
            assert_eq!(
                canonical_session_json(&model).expect("write corpus session"),
                source,
                "{path} must be exact Rust canonical bytes"
            );
            json!({"id": id, "path": path})
        });
        let documents = [
            documents[0].clone(),
            json!({
                "id": "canonical-full-surface",
                "canonical": full_surface_document(),
            }),
            json!({
                "id": "canonical-exchanged-strips",
                "canonical": exchanged_strips_document(),
            }),
        ];

        let mut floats = vec![
            f32_case("positive-zero", 0x0000_0000),
            f32_case("negative-zero", 0x8000_0000),
            f32_case("one", 0x3f80_0000),
            f32_case("negative-one", 0xbf80_0000),
            f32_case("ordinary-tenth", 0x3dcc_cccd),
            f32_case("minimum-positive-subnormal", 0x0000_0001),
            f32_case("maximum-positive-subnormal", 0x007f_ffff),
            f32_case("minimum-negative-subnormal", 0x8000_0001),
            f32_case("minimum-positive-normal", 0x0080_0000),
            f32_case("maximum-positive-finite", 0x7f7f_ffff),
            f32_case("maximum-negative-finite", 0xff7f_ffff),
            f32_case("double-rounding-positive", 0x15ae_43fd),
            f32_case("double-rounding-negative", 0x95ae_43fd),
        ];
        let mut state = 0x004d_4953_4f31_3037_u64;
        while floats.len() < 29 {
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            let bits = state.wrapping_mul(0x2545_f491_4f6c_dd1d) as u32;
            if f32::from_bits(bits).is_finite() {
                floats.push(f32_case(
                    &format!("generated-{:02}", floats.len() - 12),
                    bits,
                ));
            }
        }

        let strings = [
            string_case("ascii", "plain/ascii"),
            string_case("escapes", "\"\\\u{8}\t\n\u{c}\r\u{0}"),
            string_case("c1-controls", "\u{7f}\u{80}\u{9f}"),
            string_case("unicode", "é/中/\u{2028}/\u{2029}/🙂"),
        ];
        let artifact = json!({
            "schema": "miso.session.canonical-writer-corpus.v1",
            "documents": documents,
            "f32": floats,
            "strings": strings,
        });
        let expected = format!(
            "{}\n",
            serde_json::to_string_pretty(&artifact).expect("serialize corpus")
        );
        let path = corpus_path("fixtures/session-canonical/v1/canonical-writer-corpus.json");
        if std::env::var_os("MISO_ENGINE_UPDATE_CANONICAL_WRITER_CORPUS").is_some() {
            fs::write(&path, &expected).expect("write canonical writer corpus");
        }
        assert_eq!(
            fs::read_to_string(&path).expect("read checked canonical writer corpus"),
            expected,
            "regenerate with MISO_ENGINE_UPDATE_CANONICAL_WRITER_CORPUS=1 cargo test --locked -p session canonical_writer_corpus_is_rust_generated_and_current"
        );
    }
}
