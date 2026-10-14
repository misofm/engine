//! Slice S8 of decision 17 (#1560): the C ABI and the browser host give the same result for the
//! same control edits.
//!
//! One script of control edits, played once through the C ABI (`SET_CONTROL` inside
//! `SESSION_TRANSACTION_APPLY`, through the exported `miso_engine_v1_*` entries) and once through
//! the browser host (`AudioWorkletEngineHost` with today's command records, and, for the EQ and
//! the input filters, the prepared targets the browser's own preparers design). Both legs run
//! natively in this process, so they run the same lane width; lane-width parity stays with the G5
//! corpus and the #106 F4 session parity gate.
//!
//! Per scripted edit the test compares whether the edit was accepted or refused (with the code on
//! each side for a refusal), the path (the C ABI reply's `edit_path`, which must be `live`, or
//! `monitor` for solo; the browser has no rebuild path, so an accepted browser command is a live
//! one), and, per block, the bits of the two output planes. At the end the two hosts report the
//! same `live_values_superseded` count. The whole run repeats at 44.1, 48, 88.2 and 96 kHz.
//!
//! It absorbs #1364 gate 3 (the edits carry no per-edit ramp, so each host resolves the session's
//! own length, and one pan carries a window of its own while another does not) and #1399 (a fader,
//! a pan and an EQ parameter drain alike, with equal superseded counts), whose two narrow tests it
//! replaces.
//!
//! Bypass: both hosts step a bypass today. When the bypass crossfade (#1341) reaches one host
//! before the other this test turns red at the bypass row, which is a real parity defect; the row
//! is never weakened for it.

#![allow(unsafe_code)]

use core::ptr;

use builtins::{BuiltinLaneSelector, PreparedInputFilterTarget};
use capi::{
    ABI_VERSION as C_ABI_VERSION, BYTES_OUT_SIZE, BytesOut, COMPILE_LIMITS_SIZE, CompileLimits,
    ENGINE_CONFIG_SIZE, EVENT_LANE_RELIABLE, EngineConfig, PLANAR_OUTPUT_SIZE, Plan, PlanarOutput,
    RESULT_OK as C_RESULT_OK, SOURCE_CHUNK_SIZE, SUBMIT_REPORT_SIZE, Session, SourceChunk,
    SubmitReport,
};
use controls::{ControlAddress, Lane, Owner, Section};
use effect_contract::{PREPARED_EFFECT_TARGET_WORDS, ParameterChannel as ContractChannel};
use host_core::{
    EQ_TARGET_CAPACITY, EQ_VALUE_COUNT, EqTargetEdit, EqTargetPreparer, InputFilterEdit,
    InputFilterPreparer,
};
use host_web::{
    ABI_VERSION, AudioWorkletEngineHost, COMMAND_EFFECT_BYPASS, COMMAND_EFFECT_PARAM,
    COMMAND_FADER_DB, COMMAND_INPUT_FILTERS, COMMAND_MATRIX, COMMAND_MUTE, COMMAND_PAN,
    COMMAND_POLARITY_INVERT, COMMAND_REASON_DOMAIN, COMMAND_REASON_NONE,
    COMMAND_REASON_NOT_SOLOABLE, COMMAND_REASON_UNKNOWN_VCA, COMMAND_RECORD_BYTES,
    COMMAND_ROUTE_GAIN_DB, COMMAND_ROUTE_MATRIX, COMMAND_ROUTE_MUTE, COMMAND_SOLO, COMMAND_TRIM_DB,
    COMMAND_VCA_FADER_DB, COMMAND_VCA_MUTE, RACK_INSERTS, RACK_NOT_APPLICABLE, RESULT_OK,
    SESSION_DEFAULT_RAMP, WebBootOptions,
};
use protocol::{
    CommandPayload, CounterId, CountersRequest, DecodeScratch, DecodedSuccessResponsePayload,
    DecodedTypedResponseFrame, EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit,
    SessionRevision, StatusCode, TypedCommandFrame,
};
use session::{
    DualMonoFader, EffectIdentity, EffectParam, MatrixOrPan, ParameterChannel, ParameterUnit,
    RouteDestination, RouteSource, SendTap, SessionModel, StableId, Submix, Vca,
    canonical_session_json, parse_session_json,
};

const QUANTUM: u32 = 128;
const BLOCKS: u64 = 64;
/// The strip window of `t1`'s pan, in samples. `t2`'s pan has none (`pan_ms` applies).
const PAN_WINDOW: u32 = 200;
const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

// The builtin and own-row IDs `SET_CONTROL` addresses (`controls`' tables).
const POLARITY_INVERT: u32 = 1;
const TRIM_DB: u32 = 2;
const HPF_HZ: u32 = 3;
const LPF_HZ: u32 = 4;
const FADER_DB: u32 = 5;
const MUTE: u32 = 6;
const MATRIX_LL: u32 = 7;
const PAN: u32 = 12;
const SOLO: u32 = 2;
const ROUTE_GAIN_DB: u32 = 1;
const ROUTE_MUTE: u32 = 2;
const ROUTE_MATRIX_LL: u32 = 3;
const VCA_OFFSET_DB: u32 = 1;
const VCA_MUTE: u32 = 2;

/// A strip of the session and its browser strip index (tracks in ID order, then submixes).
#[derive(Clone, Copy)]
struct Strip {
    id: &'static str,
    index: u32,
}

const T1: Strip = Strip { id: "t1", index: 0 };
const T2: Strip = Strip { id: "t2", index: 1 };
const T3: Strip = Strip { id: "t3", index: 2 };
const BUS: Strip = Strip { id: "b", index: 3 };

/// One insert of a track: its ID, its native identity and its position in the track's inserts
/// (the browser's `effect_index`).
#[derive(Clone, Copy)]
struct Insert {
    track: Strip,
    id: &'static str,
    native: &'static str,
    position: u32,
}

const EQ: Insert = Insert {
    track: T1,
    id: "eq",
    native: "miso.parametric-eq",
    position: 0,
};
const COMPRESSOR: Insert = Insert {
    track: T1,
    id: "comp",
    native: "miso.compressor",
    position: 1,
};
const GATE: Insert = Insert {
    track: T2,
    id: "gate",
    native: "miso.gate-expander",
    position: 0,
};
const CLIP: Insert = Insert {
    track: T2,
    id: "clip",
    native: "miso.soft-clip",
    position: 1,
};
const SHAPER: Insert = Insert {
    track: T2,
    id: "shaper",
    native: "miso.transient-shaper",
    position: 2,
};
const MULTIBAND: Insert = Insert {
    track: T3,
    id: "multi",
    native: "miso.multiband-compressor",
    position: 0,
};
const LIMITER: Insert = Insert {
    track: T3,
    id: "limiter",
    native: "miso.true-peak-limiter",
    position: 1,
};
const DELAY: Insert = Insert {
    track: T3,
    id: "delay",
    native: "miso.delay",
    position: 2,
};
const INSERTS: [Insert; 8] = [
    EQ, COMPRESSOR, GATE, CLIP, SHAPER, MULTIBAND, LIMITER, DELAY,
];

/// Which lane(s) of a dual-mono value an edit writes.
#[derive(Clone, Copy)]
enum Lanes {
    Left,
    Right,
    Both,
}

impl Lanes {
    /// The browser's wire `channel` byte.
    fn byte(self) -> u8 {
        match self {
            Self::Left => 0,
            Self::Right => 1,
            Self::Both => 2,
        }
    }

    fn lane(self) -> Lane {
        match self {
            Self::Left => ParameterChannel::Left,
            Self::Right => ParameterChannel::Right,
            Self::Both => ParameterChannel::Both,
        }
    }
}

/// A VCA by its stable ID and its browser VCA index (canonical ID order).
#[derive(Clone, Copy)]
struct VcaRef {
    id: &'static str,
    index: u32,
}

const VCA: VcaRef = VcaRef { id: "v", index: 0 };
/// A VCA the session does not have: the C ABI names it by an unknown ID, the browser by an index
/// past the last VCA.
const NO_VCA: VcaRef = VcaRef {
    id: "nobody",
    index: 1,
};

/// One live edit of the script, in terms both hosts can express.
#[derive(Clone, Copy)]
enum Edit {
    Fader {
        strip: Strip,
        lanes: Lanes,
        db: f32,
    },
    Mute {
        strip: Strip,
        lanes: Lanes,
        muted: bool,
    },
    Pan {
        strip: Strip,
        left: f32,
        right: f32,
    },
    Matrix {
        strip: Strip,
        values: [f32; 4],
    },
    Trim {
        strip: Strip,
        lanes: Lanes,
        db: f32,
    },
    Polarity {
        strip: Strip,
        lanes: Lanes,
        inverted: bool,
    },
    Filters {
        strip: Strip,
        hpf_hz: f32,
        lpf_hz: f32,
    },
    /// A block-rate effect parameter, through the browser's direct command.
    Parameter {
        insert: Insert,
        parameter: u32,
        lanes: Lanes,
        value: f32,
    },
    /// A parametric EQ parameter on both lanes, through the browser's prepared target.
    EqParameter {
        parameter: u32,
        value: f32,
    },
    Bypass {
        insert: Insert,
        bypassed: bool,
    },
    SendGain {
        db: f32,
    },
    SendMute {
        muted: bool,
    },
    SendMatrix {
        values: [f32; 4],
    },
    VcaOffset {
        vca: VcaRef,
        lanes: Lanes,
        db: f32,
    },
    VcaMute {
        vca: VcaRef,
        lanes: Lanes,
        muted: bool,
    },
    Solo {
        strip: Strip,
        on: bool,
    },
}

/// How a scripted edit must end on both hosts.
#[derive(Clone, Copy, PartialEq)]
enum Outcome {
    /// Accepted: `live` on the C ABI (`monitor` for solo), admitted on the browser.
    Accepted,
    /// Accepted and a no-op: the edit repeats the committed value, so the C ABI answers
    /// `model_only` and writes no record, and the browser admits the command and writes none.
    Unchanged,
    /// Refused: the C ABI's control code and the browser's `COMMAND_REASON_*`.
    Refused { code: &'static str, reason: u32 },
}

/// One scripted edit: the block it is submitted before, its ramp length (`None` is the session's
/// own on both hosts) and how it must end.
struct Step {
    block: u64,
    edit: Edit,
    ramp: Option<u32>,
    outcome: Outcome,
}

const fn live(block: u64, edit: Edit) -> Step {
    Step {
        block,
        edit,
        ramp: None,
        outcome: Outcome::Accepted,
    }
}

const fn live_ramped(block: u64, edit: Edit, ramp: u32) -> Step {
    Step {
        block,
        edit,
        ramp: Some(ramp),
        outcome: Outcome::Accepted,
    }
}

const fn refused(block: u64, edit: Edit, code: &'static str, reason: u32) -> Step {
    Step {
        block,
        edit,
        ramp: None,
        outcome: Outcome::Refused { code, reason },
    }
}

const fn unchanged(block: u64, edit: Edit, ramp: u32) -> Step {
    Step {
        block,
        edit,
        ramp: Some(ramp),
        outcome: Outcome::Unchanged,
    }
}

/// The script. Every accepted row is live on both hosts and every edit but the two named ramps
/// gives no length. Two fader edits share block 4, two `t1` pans block 16 and two compressor
/// thresholds block 24, so each second replaces the first's write (fader, matrix and effect
/// cells all count a superseded write), and the refusals come last so the render after them is still the accepted edits'.
fn script() -> Vec<Step> {
    vec![
        // Strip level.
        live(
            4,
            Edit::Fader {
                strip: T1,
                lanes: Lanes::Both,
                db: -6.0,
            },
        ),
        live(
            4,
            Edit::Fader {
                strip: T1,
                lanes: Lanes::Both,
                db: -9.0,
            },
        ),
        live(
            6,
            Edit::Fader {
                strip: T2,
                lanes: Lanes::Left,
                db: -10.0,
            },
        ),
        live(
            8,
            Edit::Mute {
                strip: T3,
                lanes: Lanes::Both,
                muted: true,
            },
        ),
        live_ramped(
            10,
            Edit::Fader {
                strip: BUS,
                lanes: Lanes::Both,
                db: -4.0,
            },
            480,
        ),
        live_ramped(
            12,
            Edit::Fader {
                strip: T1,
                lanes: Lanes::Both,
                db: -3.0,
            },
            0,
        ),
        // Strip matrix and pan.
        live(
            14,
            Edit::Pan {
                strip: T2,
                left: -0.2,
                right: 0.7,
            },
        ),
        live(
            16,
            Edit::Pan {
                strip: T1,
                left: 0.1,
                right: -0.3,
            },
        ),
        // A second pan of the same strip before the same block: the first write is superseded
        // (the matrix cell's count, which the browser and the core must agree on).
        live(
            16,
            Edit::Pan {
                strip: T1,
                left: 0.4,
                right: -0.1,
            },
        ),
        live(
            18,
            Edit::Matrix {
                strip: T3,
                values: [0.7, 0.3, -0.2, 0.9],
            },
        ),
        // Strip input.
        live(
            20,
            Edit::Trim {
                strip: T1,
                lanes: Lanes::Both,
                db: -5.0,
            },
        ),
        live(
            21,
            Edit::Polarity {
                strip: T2,
                lanes: Lanes::Right,
                inverted: true,
            },
        ),
        live(
            22,
            Edit::Filters {
                strip: T3,
                hpf_hz: 80.0,
                lpf_hz: 8_000.0,
            },
        ),
        // Effects: one block-rate parameter of every launch effect, an EQ band gain, a bypass.
        live(
            24,
            Edit::Parameter {
                insert: COMPRESSOR,
                parameter: 1,
                lanes: Lanes::Left,
                value: -30.0,
            },
        ),
        // A second threshold of the same insert before the same block: an effect cell's
        // superseded write.
        live(
            24,
            Edit::Parameter {
                insert: COMPRESSOR,
                parameter: 1,
                lanes: Lanes::Left,
                value: -34.0,
            },
        ),
        live(
            25,
            Edit::Parameter {
                insert: GATE,
                parameter: 1,
                lanes: Lanes::Both,
                value: 0.0,
            },
        ),
        live(
            26,
            Edit::Parameter {
                insert: CLIP,
                parameter: 1,
                lanes: Lanes::Both,
                value: 12.0,
            },
        ),
        live(
            27,
            Edit::Parameter {
                insert: SHAPER,
                parameter: 1,
                lanes: Lanes::Both,
                value: 0.8,
            },
        ),
        live(
            28,
            Edit::Parameter {
                insert: MULTIBAND,
                parameter: 3,
                lanes: Lanes::Both,
                value: -40.0,
            },
        ),
        live(
            29,
            Edit::Parameter {
                insert: LIMITER,
                parameter: 1,
                lanes: Lanes::Both,
                value: -12.0,
            },
        ),
        live(
            30,
            Edit::Parameter {
                insert: DELAY,
                parameter: 4,
                lanes: Lanes::Both,
                value: 0.8,
            },
        ),
        live(
            31,
            Edit::EqParameter {
                parameter: 4,
                value: -9.0,
            },
        ),
        live(
            32,
            Edit::Bypass {
                insert: COMPRESSOR,
                bypassed: true,
            },
        ),
        // Routes: the send's gain, matrix and mute.
        live(36, Edit::SendGain { db: -6.0 }),
        live(
            38,
            Edit::SendMatrix {
                values: [0.5, 0.5, 0.2, 0.8],
            },
        ),
        live(40, Edit::SendMute { muted: true }),
        live(44, Edit::SendMute { muted: false }),
        // VCA.
        live(
            42,
            Edit::VcaOffset {
                vca: VCA,
                lanes: Lanes::Both,
                db: -3.0,
            },
        ),
        live(
            46,
            Edit::VcaMute {
                vca: VCA,
                lanes: Lanes::Left,
                muted: true,
            },
        ),
        live(
            50,
            Edit::VcaMute {
                vca: VCA,
                lanes: Lanes::Left,
                muted: false,
            },
        ),
        // Solo on, then off.
        live(
            52,
            Edit::Solo {
                strip: T2,
                on: true,
            },
        ),
        live(
            57,
            Edit::Solo {
                strip: T2,
                on: false,
            },
        ),
        // Unmute the track that block 8 muted, so its mute ramps out and back in.
        live(
            30,
            Edit::Mute {
                strip: T3,
                lanes: Lanes::Both,
                muted: false,
            },
        ),
        // Repeats: each writes the value its row already holds, one block after the write and
        // over a new ramp length, while the first ramp is still in flight. The core writes no
        // record for an equal value (`model_only`), so the ramp goes on; a host that wrote the
        // record would restart it. The send mute at block 39 is the case #1560 found: the matrix
        // ramp of block 38 is in flight, and the mute was already false.
        unchanged(
            5,
            Edit::Fader {
                strip: T1,
                lanes: Lanes::Both,
                db: -9.0,
            },
            480,
        ),
        unchanged(
            7,
            Edit::Fader {
                strip: T2,
                lanes: Lanes::Left,
                db: -10.0,
            },
            480,
        ),
        unchanged(
            9,
            Edit::Mute {
                strip: T3,
                lanes: Lanes::Both,
                muted: true,
            },
            480,
        ),
        unchanged(
            15,
            Edit::Pan {
                strip: T2,
                left: -0.2,
                right: 0.7,
            },
            480,
        ),
        unchanged(
            19,
            Edit::Matrix {
                strip: T3,
                values: [0.7, 0.3, -0.2, 0.9],
            },
            480,
        ),
        unchanged(
            21,
            Edit::Trim {
                strip: T1,
                lanes: Lanes::Both,
                db: -5.0,
            },
            480,
        ),
        unchanged(
            22,
            Edit::Polarity {
                strip: T2,
                lanes: Lanes::Right,
                inverted: true,
            },
            480,
        ),
        unchanged(37, Edit::SendGain { db: -6.0 }, 480),
        unchanged(
            39,
            Edit::SendMatrix {
                values: [0.5, 0.5, 0.2, 0.8],
            },
            480,
        ),
        unchanged(39, Edit::SendMute { muted: false }, 480),
        // Refusals: one each, in the order the table in the doc names them.
        refused(
            58,
            Edit::Fader {
                strip: T1,
                lanes: Lanes::Both,
                db: 30.0,
            },
            "control.domain",
            COMMAND_REASON_DOMAIN,
        ),
        refused(
            59,
            Edit::Solo {
                strip: BUS,
                on: true,
            },
            "control.address.unknown",
            COMMAND_REASON_NOT_SOLOABLE,
        ),
        refused(
            60,
            Edit::VcaOffset {
                vca: NO_VCA,
                lanes: Lanes::Both,
                db: -3.0,
            },
            "control.address.unknown",
            COMMAND_REASON_UNKNOWN_VCA,
        ),
    ]
}

/// The session at `rate`: tracks `t1`..`t3` over their own sources, submix `b` routed to the
/// output, `t1` sending `pre_fader` into `b` with `follows_mute`, VCA `v` holding `t2`, and one
/// instance of every launch native effect over the tracks.
fn model_at(rate: u32) -> SessionModel {
    let mut model = parse_session_json(include_str!("browser-v1/command-session.json"))
        .expect("accepted command fixture");
    model.sample_rate_hz = rate;
    model.quantum_frames = QUANTUM;
    let source = model.sources[0].clone();
    let track = model.tracks[0].clone();
    let route = model.routes[0].clone();
    // The fixture's EQ has band 1 enabled, as a low shelf at 1 kHz, so its gain is audible.
    let eq = track.inserts.effects[0].clone();
    model.sources.clear();
    model.tracks.clear();
    model.routes.clear();
    for (index, strip) in [T1, T2, T3].into_iter().enumerate() {
        let mut source = source.clone();
        source.id = StableId::parse(&format!("src-{}", strip.id)).expect("source ID");
        source.frames = u64::from(QUANTUM) * (BLOCKS + 8);
        let mut track = track.clone();
        track.id = StableId::parse(strip.id).expect("track ID");
        track.source_id = source.id.clone();
        track.fader.left_db = -2.0 - index as f32;
        track.fader.right_db = -2.0 - index as f32;
        track.matrix_or_pan = match strip.id {
            "t1" => MatrixOrPan::Pan {
                left: -0.8,
                right: 0.6,
                smoothing_samples: PAN_WINDOW,
            },
            "t2" => MatrixOrPan::Pan {
                left: -0.5,
                right: 0.4,
                smoothing_samples: 0,
            },
            _ => MatrixOrPan::Matrix {
                ll: 0.9,
                lr: 0.1,
                rl: 0.2,
                rr: 0.8,
                smoothing_samples: 0,
            },
        };
        track.inserts.effects = INSERTS
            .iter()
            .filter(|insert| insert.track.id == strip.id)
            .map(|insert| {
                let mut effect = eq.clone();
                effect.id = StableId::parse(insert.id).expect("effect ID");
                effect.identity = EffectIdentity::Native {
                    effect_id: StableId::parse(insert.native).expect("native effect ID"),
                };
                if insert.id != "eq" {
                    // Every other insert takes its default parameters, but for the gate's hold:
                    // its default (100 ms) keeps an open gate open for longer than the run's tail
                    // at 96 kHz, so a threshold edit would never be heard there.
                    effect.params.clear();
                }
                if insert.id == "gate" {
                    effect.params.push(EffectParam {
                        parameter_id: 6,
                        channel: ParameterChannel::Both,
                        unit: ParameterUnit::Milliseconds,
                        value: 0.0,
                    });
                }
                effect
            })
            .collect();
        let mut route = route.clone();
        route.id = StableId::parse(&format!("{}-main", strip.id)).expect("route ID");
        let session::RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("the fixture routes a track")
        };
        *track_id = track.id.clone();
        model.sources.push(source);
        model.tracks.push(track);
        model.routes.push(route);
    }
    let bus = StableId::parse(BUS.id).expect("submix ID");
    model
        .submixes
        .push(Submix::unity(bus.clone(), &model.console));
    let mut send = model.routes[0].clone();
    send.id = StableId::parse("t1-b").expect("route ID");
    send.source = RouteSource::Track {
        track_id: StableId::parse(T1.id).expect("track ID"),
        tap: SendTap::PreFader,
    };
    send.destination = RouteDestination::SubmixInput {
        submix_id: bus.clone(),
    };
    send.follows_mute = true;
    let mut out = model.routes[0].clone();
    out.id = StableId::parse("b-main").expect("route ID");
    out.source = RouteSource::Submix {
        submix_id: bus,
        tap: SendTap::PostPan,
    };
    model.routes.push(send);
    model.routes.push(out);
    model.vcas.push(Vca {
        id: StableId::parse(VCA.id).expect("VCA ID"),
        fader: DualMonoFader {
            left_db: 0.0,
            right_db: 0.0,
            left_mute: false,
            right_mute: false,
        },
        members: vec![StableId::parse(T2.id).expect("track ID")],
    });
    model
}

/// A deterministic, never-silent sample for source `feed`, lane `lane`, absolute frame `frame`.
fn sample(feed: u64, lane: u64, frame: u64) -> f32 {
    let mut z = (feed + 1)
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(lane.wrapping_mul(0xBF58_476D_1CE4_E5B9))
        .wrapping_add(frame.wrapping_mul(0x94D0_49BB_1331_11EB));
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    let unit = (z >> 40) as f32 / (1_u64 << 24) as f32 * 2.0 - 1.0;
    // Source 1 (`t2`) is quiet, so a gate threshold edit has a level to act on; the others are loud.
    unit * if feed == 1 { 0.25 } else { 0.8 }
}

fn planes(feed: u64, block: u64) -> [Vec<f32>; 2] {
    let first = block * u64::from(QUANTUM);
    [0, 1].map(|lane| {
        (0..u64::from(QUANTUM))
            .map(|offset| sample(feed, lane, first + offset))
            .collect()
    })
}

fn bits(pcm: &[f32]) -> Vec<u32> {
    pcm.iter().map(|sample| sample.to_bits()).collect()
}

// ---------------------------------------------------------------------------------------------
// The browser leg
// ---------------------------------------------------------------------------------------------

/// The browser leg: native `AudioWorkletEngineHost` with commands staged as the worklet stages
/// them, one submission per edit.
struct Browser {
    host: AudioWorkletEngineHost,
    rate: u32,
}

/// What the browser said about one submission.
#[derive(Debug, PartialEq)]
struct BrowserVerdict {
    result: u32,
    reason: u32,
    admitted: u32,
}

impl Browser {
    fn new(model: &SessionModel) -> Self {
        let document = canonical_session_json(model).expect("canonical session");
        let options = WebBootOptions {
            require_sample_rate_hz: model.sample_rate_hz,
            require_quantum_frames: QUANTUM,
            source_ring_frames: QUANTUM * 2,
            live_control_command_queue_records: 64,
            ..WebBootOptions::explicit_defaults()
        };
        let host = AudioWorkletEngineHost::boot(document.as_bytes(), options)
            .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.diagnostic())));
        Self {
            host,
            rate: model.sample_rate_hz,
        }
    }

    /// Stage one record at slot 0.
    #[allow(clippy::too_many_arguments)]
    fn stage(
        &mut self,
        kind: u32,
        rack: u8,
        channel: u8,
        index: u32,
        effect_index: u32,
        parameter_id: u32,
        ramp: u32,
        values: [f32; 4],
    ) {
        let staging = self.host.command_staging_mut().expect("command staging");
        let record = &mut staging[..COMMAND_RECORD_BYTES as usize];
        record.fill(0);
        record[0] = u8::try_from(kind).expect("frozen kind");
        record[1] = rack;
        record[2] = channel;
        record[4..8].copy_from_slice(&index.to_le_bytes());
        record[8..12].copy_from_slice(&effect_index.to_le_bytes());
        record[12..16].copy_from_slice(&parameter_id.to_le_bytes());
        record[16..20].copy_from_slice(&ramp.to_le_bytes());
        for (slot, value) in values.iter().enumerate() {
            record[24 + slot * 4..28 + slot * 4].copy_from_slice(&value.to_le_bytes());
        }
    }

    fn verdict(&self, result: u32) -> BrowserVerdict {
        let report = self.host.command_report();
        assert_eq!(report.result, result, "the report names the call's result");
        BrowserVerdict {
            result,
            reason: report.reason,
            admitted: report.admitted,
        }
    }

    fn submit(&mut self) -> BrowserVerdict {
        let result = self.host.submit_commands(1);
        self.verdict(result)
    }

    /// One edit as the browser takes it, with `ramp` as the record's length word.
    fn edit(&mut self, edit: Edit, ramp: Option<u32>) -> BrowserVerdict {
        let ramp_word = ramp.unwrap_or(SESSION_DEFAULT_RAMP);
        let none = RACK_NOT_APPLICABLE;
        match edit {
            Edit::Fader { strip, lanes, db } => {
                self.stage(
                    COMMAND_FADER_DB,
                    none,
                    lanes.byte(),
                    strip.index,
                    0,
                    0,
                    ramp_word,
                    [db, 0.0, 0.0, 0.0],
                );
            }
            Edit::Mute {
                strip,
                lanes,
                muted,
            } => {
                let value = f32::from(u8::from(muted));
                self.stage(
                    COMMAND_MUTE,
                    none,
                    lanes.byte(),
                    strip.index,
                    0,
                    0,
                    ramp_word,
                    [value, 0.0, 0.0, 0.0],
                );
            }
            Edit::Pan { strip, left, right } => {
                self.stage(
                    COMMAND_PAN,
                    none,
                    255,
                    strip.index,
                    0,
                    0,
                    ramp_word,
                    [left, right, 0.0, 0.0],
                );
            }
            Edit::Matrix { strip, values } => {
                self.stage(
                    COMMAND_MATRIX,
                    none,
                    255,
                    strip.index,
                    0,
                    0,
                    ramp_word,
                    values,
                );
            }
            Edit::Trim { strip, lanes, db } => {
                self.stage(
                    COMMAND_TRIM_DB,
                    none,
                    lanes.byte(),
                    strip.index,
                    0,
                    0,
                    ramp_word,
                    [db, 0.0, 0.0, 0.0],
                );
            }
            Edit::Polarity {
                strip,
                lanes,
                inverted,
            } => {
                let value = f32::from(u8::from(inverted));
                self.stage(
                    COMMAND_POLARITY_INVERT,
                    none,
                    lanes.byte(),
                    strip.index,
                    0,
                    0,
                    ramp_word,
                    [value, 0.0, 0.0, 0.0],
                );
            }
            Edit::Filters {
                strip,
                hpf_hz,
                lpf_hz,
            } => return self.filters(strip, hpf_hz, lpf_hz),
            Edit::Parameter {
                insert,
                parameter,
                lanes,
                value,
            } => {
                // An effect parameter carries no length word: the effect smooths it itself.
                self.stage(
                    COMMAND_EFFECT_PARAM,
                    RACK_INSERTS,
                    lanes.byte(),
                    insert.track.index,
                    insert.position,
                    parameter,
                    0,
                    [value, 0.0, 0.0, 0.0],
                );
            }
            Edit::EqParameter { parameter, value } => return self.eq_parameter(parameter, value),
            Edit::Bypass { insert, bypassed } => {
                let value = f32::from(u8::from(bypassed));
                self.stage(
                    COMMAND_EFFECT_BYPASS,
                    RACK_INSERTS,
                    255,
                    insert.track.index,
                    insert.position,
                    0,
                    0,
                    [value, 0.0, 0.0, 0.0],
                );
            }
            // The one send is live route 0.
            Edit::SendGain { db } => {
                self.stage(
                    COMMAND_ROUTE_GAIN_DB,
                    none,
                    255,
                    0,
                    0,
                    0,
                    ramp_word,
                    [db, 0.0, 0.0, 0.0],
                );
            }
            Edit::SendMute { muted } => {
                let value = f32::from(u8::from(muted));
                self.stage(
                    COMMAND_ROUTE_MUTE,
                    none,
                    255,
                    0,
                    0,
                    0,
                    ramp_word,
                    [value, 0.0, 0.0, 0.0],
                );
            }
            Edit::SendMatrix { values } => {
                self.stage(COMMAND_ROUTE_MATRIX, none, 255, 0, 0, 0, ramp_word, values);
            }
            Edit::VcaOffset { vca, lanes, db } => {
                self.stage(
                    COMMAND_VCA_FADER_DB,
                    none,
                    lanes.byte(),
                    vca.index,
                    0,
                    0,
                    ramp_word,
                    [db, 0.0, 0.0, 0.0],
                );
            }
            Edit::VcaMute { vca, lanes, muted } => {
                let value = f32::from(u8::from(muted));
                self.stage(
                    COMMAND_VCA_MUTE,
                    none,
                    lanes.byte(),
                    vca.index,
                    0,
                    0,
                    ramp_word,
                    [value, 0.0, 0.0, 0.0],
                );
            }
            Edit::Solo { strip, on } => {
                let value = f32::from(u8::from(on));
                self.stage(
                    COMMAND_SOLO,
                    none,
                    255,
                    strip.index,
                    0,
                    0,
                    ramp_word,
                    [value, 0.0, 0.0, 0.0],
                );
            }
        }
        self.submit()
    }

    /// The pair of input filters on both lanes, through the targets the browser's preparer designs
    /// from the owner's committed seeds.
    fn filters(&mut self, strip: Strip, hpf_hz: f32, lpf_hz: f32) -> BrowserVerdict {
        self.stage(
            COMMAND_INPUT_FILTERS,
            RACK_NOT_APPLICABLE,
            2,
            strip.index,
            0,
            0,
            0,
            [hpf_hz, lpf_hz, 0.0, 0.0],
        );
        assert_eq!(self.host.copy_input_filter_config(strip.index), RESULT_OK);
        let config = self.host.eq_target_config().expect("input filter config");
        let sample_rate = u32::from_le_bytes(config[8..12].try_into().expect("rate"));
        let generation = u64::from_le_bytes(config[16..24].try_into().expect("generation"));
        let base_revision = u64::from_le_bytes(config[24..32].try_into().expect("revision"));
        let mut seeds = [0.0_f32; 4];
        for (index, seed) in seeds.iter_mut().enumerate() {
            let offset = 32 + index * 4;
            *seed = f32::from_bits(u32::from_le_bytes(
                config[offset..offset + 4].try_into().expect("seed"),
            ));
        }
        let mut targets = [PreparedInputFilterTarget {
            lanes: BuiltinLaneSelector::Both,
            section: 0,
            pair: [0.0; 2],
            coefficients: [0.0; 6],
        }; 4];
        let (_, count) = InputFilterPreparer
            .prepare(
                sample_rate,
                &seeds,
                &[InputFilterEdit {
                    parameter_id: 0,
                    channel: ContractChannel::Both,
                    value0: hpf_hz,
                    value1: lpf_hz,
                }],
                &mut targets,
            )
            .expect("prepared input filter targets");
        let companion = self
            .host
            .prepared_companion_mut()
            .expect("prepared companion");
        companion.fill(0);
        companion[0..4].copy_from_slice(&24_u32.to_le_bytes());
        companion[4..8].copy_from_slice(&ABI_VERSION.to_le_bytes());
        companion[8..16].copy_from_slice(&generation.to_le_bytes());
        companion[16..20].copy_from_slice(&(count as u32).to_le_bytes());
        for (index, target) in targets[..count].iter().enumerate() {
            let record = &mut companion[24 + index * 80..104 + index * 80];
            record[0..4].copy_from_slice(&strip.index.to_le_bytes());
            record[4..8].copy_from_slice(&u32::from(RACK_NOT_APPLICABLE).to_le_bytes());
            record[16..24].copy_from_slice(&base_revision.to_le_bytes());
            record[24..28].copy_from_slice(&target.section.to_le_bytes());
            let lanes: u32 = match target.lanes {
                BuiltinLaneSelector::Left => 0,
                BuiltinLaneSelector::Right => 1,
                BuiltinLaneSelector::Both => 2,
            };
            record[28..32].copy_from_slice(&lanes.to_le_bytes());
            record[32..36].copy_from_slice(&target.pair[0].to_bits().to_le_bytes());
            record[36..40].copy_from_slice(&target.pair[1].to_bits().to_le_bytes());
            for (word, value) in target.coefficients.iter().copied().enumerate() {
                record[56 + word * 4..60 + word * 4]
                    .copy_from_slice(&value.to_bits().to_le_bytes());
            }
        }
        let bytes = u32::try_from(24 + count * 80).expect("companion size");
        let result = self.host.submit_prepared_commands(1, bytes);
        self.verdict(result)
    }

    /// One EQ parameter on both lanes: the record, the designed target and the prepared
    /// submission.
    fn eq_parameter(&mut self, parameter: u32, value: f32) -> BrowserVerdict {
        self.stage(
            COMMAND_EFFECT_PARAM,
            RACK_INSERTS,
            2,
            EQ.track.index,
            EQ.position,
            parameter,
            0,
            [value, 0.0, 0.0, 0.0],
        );
        assert_eq!(
            self.host
                .copy_eq_target_config(EQ.track.index, u32::from(RACK_INSERTS), EQ.position),
            RESULT_OK
        );
        let config = self.host.eq_target_config().expect("EQ target config");
        let generation = u64::from_le_bytes(config[16..24].try_into().expect("generation"));
        let base_revision = u64::from_le_bytes(config[24..32].try_into().expect("revision"));
        let mut seeds = [0.0_f32; EQ_VALUE_COUNT];
        for (index, seed) in seeds.iter_mut().enumerate() {
            let offset = 32 + index * 4;
            *seed = f32::from_bits(u32::from_le_bytes(
                config[offset..offset + 4].try_into().expect("seed"),
            ));
        }
        let mut targets = [effect_contract::PreparedEffectTarget {
            slot: 0,
            channel: ContractChannel::Both,
            words: [0; PREPARED_EFFECT_TARGET_WORDS],
        }; EQ_TARGET_CAPACITY];
        let (_, count) = EqTargetPreparer::new(
            host_core::parametric_eq_target_preparation_factory().expect("EQ capability"),
        )
        .expect("EQ target preparer")
        .prepare(
            self.rate,
            &seeds,
            &[EqTargetEdit {
                parameter_id: parameter,
                channel: ContractChannel::Both,
                value,
            }],
            &mut targets,
        )
        .expect("prepared EQ target");
        assert_eq!(count, 1, "one edit touches one EQ section");
        let companion = self
            .host
            .prepared_companion_mut()
            .expect("prepared companion");
        companion.fill(0);
        companion[0..4].copy_from_slice(&24_u32.to_le_bytes());
        companion[4..8].copy_from_slice(&ABI_VERSION.to_le_bytes());
        companion[8..16].copy_from_slice(&generation.to_le_bytes());
        companion[16..20].copy_from_slice(&(count as u32).to_le_bytes());
        let record = &mut companion[24..104];
        record[0..4].copy_from_slice(&EQ.track.index.to_le_bytes());
        record[4..8].copy_from_slice(&u32::from(RACK_INSERTS).to_le_bytes());
        record[8..12].copy_from_slice(&EQ.position.to_le_bytes());
        record[16..24].copy_from_slice(&base_revision.to_le_bytes());
        record[24..28].copy_from_slice(&targets[0].slot.to_le_bytes());
        record[28..32].copy_from_slice(&2_u32.to_le_bytes());
        for (index, word) in targets[0].words.iter().enumerate() {
            let offset = 32 + index * 4;
            record[offset..offset + 4].copy_from_slice(&word.to_le_bytes());
        }
        let result = self.host.submit_prepared_commands(1, 104);
        self.verdict(result)
    }

    fn step(&mut self, block: u64) -> Vec<u32> {
        for (feed, strip) in [T1, T2, T3].into_iter().enumerate() {
            let [left, right] = planes(feed as u64, block);
            assert_eq!(
                self.host.submit_source(
                    format!("src-{}", strip.id).as_bytes(),
                    1,
                    block * u64::from(QUANTUM),
                    self.rate,
                    &[&left, &right],
                    QUANTUM,
                    false,
                ),
                RESULT_OK,
                "browser source {}",
                strip.id
            );
        }
        assert_eq!(self.host.render_next(), RESULT_OK, "browser render");
        bits(self.host.output_pcm().expect("browser output"))
    }

    fn superseded(&self) -> u64 {
        self.host.status().live_values_superseded
    }
}

// ---------------------------------------------------------------------------------------------
// The C ABI leg
// ---------------------------------------------------------------------------------------------

fn limits() -> CompileLimits {
    CompileLimits {
        struct_size: COMPILE_LIMITS_SIZE,
        source_ring_frames: QUANTUM * 2,
        maximum_automation_spans_per_block: 128,
        reserved0: 0,
        maximum_document_bytes: 1_000_000,
        maximum_diagnostic_bytes: 4_096,
        maximum_tracks: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 100_000_000,
        maximum_source_total_bytes: 10_000_000,
        maximum_source_overhead_bytes: 10_000_000,
        maximum_effect_state_bytes: 100_000_000,
        maximum_effect_scratch_bytes: 100_000_000,
        maximum_builtin_retained_bytes: 100_000_000,
        maximum_capi_retained_bytes: 10_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
        maximum_control_frame_bytes: 4_096,
        maximum_replay_bytes: 8_192,
        maximum_replay_entries: 16,
        maximum_submixes: 100,
        maximum_vcas: 100,
        reserved: [0; 2],
    }
}

/// What the C ABI said about one transaction.
#[derive(Debug, PartialEq)]
enum CVerdict {
    /// Committed, with the path the reply names.
    Applied(EditPath),
    /// Refused with the control codes of its diagnostics.
    Refused(Vec<String>),
}

/// The C ABI leg: exported entries, edits as `SET_CONTROL` transactions.
struct CAbi {
    session: *mut Session,
    plan: *mut Plan,
    rate: u32,
    revision: u64,
    request: u64,
}

fn address(owner: Owner, section: Section, control: u32, lane: Lane) -> ControlAddress {
    ControlAddress {
        owner,
        section,
        control,
        lane,
    }
}

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable ID")
}

impl CAbi {
    fn new(model: &SessionModel) -> Self {
        let document = canonical_session_json(model).expect("canonical session");
        let config = EngineConfig {
            struct_size: ENGINE_CONFIG_SIZE,
            abi_version: C_ABI_VERSION,
            reserved: [0; 4],
        };
        let mut engine = ptr::null_mut();
        let mut session = ptr::null_mut();
        let mut plan = ptr::null_mut();
        let mut storage = [0_u8; 4_096];
        let mut diagnostics = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: storage.as_mut_ptr(),
            capacity_bytes: storage.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: Every descriptor and output location lives for the complete call, and the engine
        // handle is destroyed once after compilation, as the C ABI documents.
        unsafe {
            assert_eq!(
                capi::miso_engine_v1_engine_create(&config, &mut engine),
                C_RESULT_OK
            );
            assert_eq!(
                capi::miso_engine_v1_compile_session(
                    engine,
                    document.as_ptr(),
                    document.len() as u64,
                    &limits(),
                    &mut diagnostics,
                    &mut session,
                    &mut plan,
                ),
                C_RESULT_OK,
                "{}",
                String::from_utf8_lossy(&storage[..diagnostics.required_bytes as usize])
            );
            capi::miso_engine_v1_engine_destroy(engine);
        }
        Self {
            session,
            plan,
            rate: model.sample_rate_hz,
            revision: model.revision,
            request: 0,
        }
    }

    fn strip_address(strip: Strip, control: u32, lanes: Lanes) -> ControlAddress {
        address(
            Owner::Strip(id(strip.id)),
            Section::Builtins,
            control,
            lanes.lane(),
        )
    }

    /// The `SET_CONTROL` edits of one scripted edit.
    fn edits(edit: Edit, ramp: Option<u32>) -> Vec<SessionEdit> {
        let set = |address: ControlAddress, value: f32| SessionEdit::SetControl {
            address,
            value,
            ramp_samples: ramp,
        };
        let route = || Owner::Route(id("t1-b"));
        let toggle = |on: bool| f32::from(u8::from(on));
        match edit {
            Edit::Fader { strip, lanes, db } => {
                vec![set(Self::strip_address(strip, FADER_DB, lanes), db)]
            }
            Edit::Mute {
                strip,
                lanes,
                muted,
            } => {
                vec![set(Self::strip_address(strip, MUTE, lanes), toggle(muted))]
            }
            // A pan is one value per lane in the control list; the browser's record carries both.
            Edit::Pan { strip, left, right } => vec![
                set(Self::strip_address(strip, PAN, Lanes::Left), left),
                set(Self::strip_address(strip, PAN, Lanes::Right), right),
            ],
            Edit::Matrix { strip, values } => values
                .into_iter()
                .zip(MATRIX_LL..)
                .map(|(value, control)| {
                    set(Self::strip_address(strip, control, Lanes::Both), value)
                })
                .collect(),
            Edit::Trim { strip, lanes, db } => {
                vec![set(Self::strip_address(strip, TRIM_DB, lanes), db)]
            }
            Edit::Polarity {
                strip,
                lanes,
                inverted,
            } => vec![set(
                Self::strip_address(strip, POLARITY_INVERT, lanes),
                toggle(inverted),
            )],
            Edit::Filters {
                strip,
                hpf_hz,
                lpf_hz,
            } => vec![
                set(Self::strip_address(strip, HPF_HZ, Lanes::Both), hpf_hz),
                set(Self::strip_address(strip, LPF_HZ, Lanes::Both), lpf_hz),
            ],
            Edit::Parameter {
                insert,
                parameter,
                lanes,
                value,
            } => vec![set(
                address(
                    Owner::Strip(id(insert.track.id)),
                    Section::Inserts(id(insert.id)),
                    parameter,
                    lanes.lane(),
                ),
                value,
            )],
            Edit::EqParameter { parameter, value } => vec![set(
                address(
                    Owner::Strip(id(EQ.track.id)),
                    Section::Inserts(id(EQ.id)),
                    parameter,
                    ParameterChannel::Both,
                ),
                value,
            )],
            Edit::Bypass { insert, bypassed } => vec![set(
                address(
                    Owner::Strip(id(insert.track.id)),
                    Section::Inserts(id(insert.id)),
                    controls::BYPASS_CONTROL,
                    ParameterChannel::Both,
                ),
                toggle(bypassed),
            )],
            Edit::SendGain { db } => vec![set(
                address(route(), Section::Own, ROUTE_GAIN_DB, ParameterChannel::Both),
                db,
            )],
            Edit::SendMute { muted } => vec![set(
                address(route(), Section::Own, ROUTE_MUTE, ParameterChannel::Both),
                toggle(muted),
            )],
            Edit::SendMatrix { values } => values
                .into_iter()
                .zip(ROUTE_MATRIX_LL..)
                .map(|(value, control)| {
                    set(
                        address(route(), Section::Own, control, ParameterChannel::Both),
                        value,
                    )
                })
                .collect(),
            Edit::VcaOffset { vca, lanes, db } => vec![set(
                address(
                    Owner::Vca(id(vca.id)),
                    Section::Own,
                    VCA_OFFSET_DB,
                    lanes.lane(),
                ),
                db,
            )],
            Edit::VcaMute { vca, lanes, muted } => vec![set(
                address(Owner::Vca(id(vca.id)), Section::Own, VCA_MUTE, lanes.lane()),
                toggle(muted),
            )],
            Edit::Solo { strip, on } => vec![set(
                address(
                    Owner::Strip(id(strip.id)),
                    Section::Own,
                    SOLO,
                    ParameterChannel::Both,
                ),
                toggle(on),
            )],
        }
    }

    /// Send one frame and decode the answer.
    fn send(&mut self, payload: CommandPayload<'_>, expected: ExpectedRevision) -> Vec<u8> {
        self.request += 1;
        let mut bytes = vec![0_u8; 4_096];
        let length = ProtocolCodec::default()
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(self.request).expect("nonzero request"),
                    expected_revision: expected,
                    payload,
                },
                &mut bytes,
            )
            .expect("command frame");
        bytes.truncate(length);
        let mut response = vec![0_u8; 4_096];
        let mut out = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: response.as_mut_ptr(),
            capacity_bytes: response.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: The session is live and used by this thread alone; the buffers are owned.
        let code = unsafe {
            capi::miso_engine_v1_submit_command(
                self.session,
                bytes.as_ptr(),
                bytes.len() as u64,
                &mut out,
            )
        };
        assert_eq!(code, C_RESULT_OK, "C ABI command");
        response.truncate(out.required_bytes as usize);
        self.drain_reliable();
        response
    }

    /// One scripted edit as one transaction at the committed revision.
    fn edit(&mut self, edit: Edit, ramp: Option<u32>) -> CVerdict {
        let edits = Self::edits(edit, ramp);
        let response = self.send(
            CommandPayload::SessionTransactionApply(&edits),
            ExpectedRevision::Exact(SessionRevision(self.revision)),
        );
        let mut fields = [0_u16; 64];
        match ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .expect("transaction response")
        {
            DecodedTypedResponseFrame::Success { header, payload } => {
                let DecodedSuccessResponsePayload::SessionTransactionApplied(applied) = payload
                else {
                    panic!("expected a transaction success")
                };
                assert_eq!(header.status, StatusCode::Ok);
                self.revision = header.revision.0;
                CVerdict::Applied(applied.edit_path)
            }
            DecodedTypedResponseFrame::NonOk { header, payload } => {
                assert_eq!(header.status, StatusCode::ValidationFailed);
                CVerdict::Refused(
                    payload
                        .diagnostics
                        .into_iter()
                        .map(|diagnostic| diagnostic.code.to_string())
                        .collect(),
                )
            }
        }
    }

    /// Drain the reliable lane so a later commit is never backpressured by an unread event.
    fn drain_reliable(&mut self) {
        loop {
            let mut event = [0_u8; 4_096];
            let mut out = BytesOut {
                struct_size: BYTES_OUT_SIZE,
                reserved0: 0,
                data: event.as_mut_ptr(),
                capacity_bytes: event.len() as u64,
                required_bytes: 0,
            };
            // SAFETY: The session is live and used by this thread alone; the buffer is owned.
            let code = unsafe {
                capi::miso_engine_v1_dequeue_event(self.session, EVENT_LANE_RELIABLE, &mut out)
            };
            assert_eq!(code, C_RESULT_OK);
            if out.required_bytes == 0 {
                break;
            }
        }
    }

    fn superseded(&mut self) -> u64 {
        let response = self.send(
            CommandPayload::CountersGet(&CountersRequest {
                all: false,
                ids: vec![CounterId::LiveValuesSuperseded as u32],
            }),
            ExpectedRevision::Any,
        );
        let mut fields = [0_u16; 64];
        let DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::CounterSnapshot(snapshot),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .expect("counters response")
        else {
            panic!("expected a counter snapshot")
        };
        snapshot
            .values
            .iter()
            .find(|value| value.id == CounterId::LiveValuesSuperseded)
            .expect("the counter is in the snapshot")
            .value
    }

    fn step(&mut self, block: u64) -> Vec<u32> {
        for (feed, strip) in [T1, T2, T3].into_iter().enumerate() {
            let [left, right] = planes(feed as u64, block);
            let pointers = [left.as_ptr(), right.as_ptr()];
            let chunk = SourceChunk {
                struct_size: SOURCE_CHUNK_SIZE,
                sample_rate_hz: self.rate,
                generation: 1,
                start_frame: block * u64::from(QUANTUM),
                planes: pointers.as_ptr(),
                plane_count: 2,
                frames: QUANTUM,
                end_of_region: 0,
                reserved0: 0,
            };
            let mut report = SubmitReport {
                struct_size: SUBMIT_REPORT_SIZE,
                reserved0: 0,
                accepted_frames: 0,
                cumulative_written_frames: 0,
                active_generation: 0,
            };
            let name = format!("src-{}", strip.id);
            // SAFETY: The session is live and used by this thread alone; the chunk's planes and
            // the report are owned storage that outlives the synchronous call.
            let code = unsafe {
                capi::miso_engine_v1_source_submit_planar_f32(
                    self.session,
                    name.as_ptr(),
                    name.len() as u64,
                    &chunk,
                    &mut report,
                )
            };
            assert_eq!(code, C_RESULT_OK, "C ABI source {}", strip.id);
            assert_eq!(report.accepted_frames, u64::from(QUANTUM));
        }
        let mut pcm = vec![f32::NAN; QUANTUM as usize * 2];
        let output = PlanarOutput {
            struct_size: PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames: QUANTUM,
            plane_stride_samples: QUANTUM,
            reserved: [0; 2],
        };
        // SAFETY: This thread is the plan's only renderer; `output` is owned storage.
        let code = unsafe {
            capi::miso_engine_v1_render_f32_planar(self.plan, block * u64::from(QUANTUM), &output)
        };
        assert_eq!(code, C_RESULT_OK, "C ABI render");
        bits(&pcm)
    }
}

impl Drop for CAbi {
    fn drop(&mut self) {
        // SAFETY: These are the exact live handles `new` returned, destroyed once.
        unsafe {
            capi::miso_engine_v1_session_destroy(self.session);
            capi::miso_engine_v1_plan_destroy(self.plan);
        }
    }
}

// ---------------------------------------------------------------------------------------------
// The comparison
// ---------------------------------------------------------------------------------------------

/// One run of `steps` on both legs. Returns the browser's blocks and the final superseded count
/// (equal on both legs, asserted here).
fn run(rate: u32, steps: &[Step]) -> (Vec<Vec<u32>>, u64) {
    let model = model_at(rate);
    let mut browser = Browser::new(&model);
    let mut c_abi = CAbi::new(&model);
    let mut blocks = Vec::new();
    for block in 0..BLOCKS {
        for (position, step) in steps.iter().enumerate().filter(|(_, s)| s.block == block) {
            let label = format!("{rate} Hz: step {position} before block {block}");
            let b = browser.edit(step.edit, step.ramp);
            let c = c_abi.edit(step.edit, step.ramp);
            match step.outcome {
                Outcome::Accepted => {
                    let path = if matches!(step.edit, Edit::Solo { .. }) {
                        EditPath::Monitor
                    } else {
                        EditPath::Live
                    };
                    assert_eq!(c, CVerdict::Applied(path), "{label}: the C ABI path");
                    assert_eq!(
                        b,
                        BrowserVerdict {
                            result: RESULT_OK,
                            reason: COMMAND_REASON_NONE,
                            admitted: 1,
                        },
                        "{label}: the browser admits the command live"
                    );
                }
                Outcome::Unchanged => {
                    assert_eq!(
                        c,
                        CVerdict::Applied(if matches!(step.edit, Edit::Solo { .. }) {
                            EditPath::Monitor
                        } else {
                            EditPath::ModelOnly
                        }),
                        "{label}: the C ABI path"
                    );
                    assert_eq!(
                        b,
                        BrowserVerdict {
                            result: RESULT_OK,
                            reason: COMMAND_REASON_NONE,
                            admitted: 1,
                        },
                        "{label}: the browser admits the command"
                    );
                }
                Outcome::Refused { code, reason } => {
                    assert!(
                        matches!(&c, CVerdict::Refused(codes) if codes.iter().any(|c| c == code)),
                        "{label}: the C ABI refuses with {code}, got {c:?}"
                    );
                    assert_eq!(b.admitted, 0, "{label}: the browser admits nothing");
                    assert_ne!(b.result, RESULT_OK, "{label}: the browser refuses");
                    assert_eq!(b.reason, reason, "{label}: the browser's reason");
                }
            }
        }
        let (b, c) = (browser.step(block), c_abi.step(block));
        let differing = b.iter().zip(&c).filter(|(x, y)| x != y).count();
        assert_eq!(
            differing, 0,
            "{rate} Hz: block {block} differs between the browser and the C ABI in {differing} samples"
        );
        blocks.push(b);
    }
    let superseded = c_abi.superseded();
    assert_eq!(
        browser.superseded(),
        superseded,
        "{rate} Hz: the browser status reports the superseded count the C ABI reports"
    );
    (blocks, superseded)
}

/// The same script on both hosts: every edit ends alike, every block has the same bits, the
/// superseded counts are equal.
///
/// Red if a value lowers to a different lane or cell, ramps over a different length, composes
/// with a VCA or a solo differently, drains differently, or is refused with a different code on
/// one host. The control run proves the script is heard: the edits move the output away from a
/// run with no edits, and dropping any one edit changes some block at 48 kHz.
#[test]
fn both_hosts_give_the_same_result_for_the_same_control_edits() {
    let steps = script();
    for rate in RATES {
        let (blocks, superseded) = run(rate, &steps);
        assert!(
            superseded >= 1,
            "{rate} Hz: the two fader edits of block 4 replace one write"
        );
        let (untouched, _) = run(rate, &[]);
        let moved = blocks
            .iter()
            .zip(&untouched)
            .filter(|(edited, plain)| edited != plain)
            .count();
        assert!(
            moved >= 40,
            "{rate} Hz: only {moved} blocks differ from the unedited run"
        );
    }
}

/// The browser leg alone over `steps`: its blocks. The audibility check below compares scripts
/// that differ by one edit, which the C ABI would answer `model_only` when the edit left out was
/// the one that made a later edit a change.
fn browser_blocks(rate: u32, steps: &[Step]) -> Vec<Vec<u32>> {
    let mut browser = Browser::new(&model_at(rate));
    (0..BLOCKS)
        .map(|block| {
            for step in steps.iter().filter(|step| step.block == block) {
                browser.edit(step.edit, step.ramp);
            }
            browser.step(block)
        })
        .collect()
}

/// Whether `later`, submitted in the same block, writes the cell that `earlier` writes.
fn replaces(earlier: &Edit, later: &Edit) -> bool {
    match (earlier, later) {
        (Edit::Fader { strip: a, .. }, Edit::Fader { strip: b, .. })
        | (Edit::Pan { strip: a, .. }, Edit::Pan { strip: b, .. }) => a.id == b.id,
        (
            Edit::Parameter {
                insert: a,
                parameter: pa,
                ..
            },
            Edit::Parameter {
                insert: b,
                parameter: pb,
                ..
            },
        ) => a.id == b.id && pa == pb,
        _ => false,
    }
}

/// Every accepted edit changes the render on its own: the script with that edit left out differs
/// from the full script in some block, so no row of the parity run is a silent no-op on both hosts
/// (a row that edited nothing would pass the comparison above on any defect).
#[test]
fn every_scripted_edit_is_heard() {
    let rate = 48_000;
    // The repeats are left out: with the edit they repeat omitted, a repeat is a change on the
    // browser leg alone, which is not what this test asks.
    let full: Vec<Step> = script()
        .into_iter()
        .filter(|step| !matches!(step.outcome, Outcome::Unchanged))
        .collect();
    let all = browser_blocks(rate, &full);
    for omitted in 0..full.len() {
        // A step that a later step of the same block replaces (same strip's fader, same strip's
        // pan, same insert parameter) is superseded by design, before any render reads it: that
        // is what the equal superseded counts check, so it is never heard.
        if !matches!(full[omitted].outcome, Outcome::Accepted)
            || full[omitted + 1..].iter().any(|later| {
                later.block == full[omitted].block && replaces(&full[omitted].edit, &later.edit)
            })
        {
            continue;
        }
        let rest: Vec<Step> = full
            .iter()
            .enumerate()
            .filter(|(position, _)| *position != omitted)
            .map(|(_, step)| Step {
                block: step.block,
                edit: step.edit,
                ramp: step.ramp,
                outcome: step.outcome,
            })
            .collect();
        assert!(
            all != browser_blocks(rate, &rest),
            "step {omitted} (block {}) changes nothing: it is not heard",
            full[omitted].block
        );
    }
}
