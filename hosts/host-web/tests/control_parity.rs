//! Slice S8 of decision 17 (#1560): the C ABI and the browser host give the same result for the
//! same control edits.
//!
//! One script of control edits, played once through the C ABI (`SET_CONTROL` inside
//! `SESSION_TRANSACTION_APPLY`, through the exported `miso_engine_v1_*` entries) and once through
//! the browser host (`AudioWorkletEngineHost`, through the generic command-frame entry of issue
//! #1562a D1: the very same frames, unchanged, as `SessionState::command` receives them). Both
//! legs run natively in this process, so they run the same lane width; lane-width parity stays
//! with the G5 corpus and the #106 F4 session parity gate.
//!
//! Per scripted edit the test compares the verdict of the two hosts whole: committed with the same
//! path (`live`, or `monitor` for solo) and the same revision, or refused with the same control
//! codes. Per block it compares the bits of the two output planes. At the end the two hosts report
//! the same `live_values_superseded` count, and every page of `CONTROL_LIST_GET` and
//! `CONTROL_STATE_GET` and the `SESSION_SNAPSHOT_GET` bytes are byte-identical. The whole run
//! repeats at 44.1, 48, 88.2 and 96 kHz. The rebuild part of the browser's parity (a
//! `delay_samples` edit) is #1562b's: until then the browser refuses it `web.apply.rebuild_pending`.
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

use capi::{
    ABI_VERSION as C_ABI_VERSION, BYTES_OUT_SIZE, BytesOut, COMPILE_LIMITS_SIZE, CompileLimits,
    ENGINE_CONFIG_SIZE, EVENT_LANE_RELIABLE, EngineConfig, PLANAR_OUTPUT_SIZE, Plan, PlanarOutput,
    RESULT_OK as C_RESULT_OK, SOURCE_CHUNK_SIZE, SUBMIT_REPORT_SIZE, Session, SourceChunk,
    SubmitReport,
};
use controls::{ControlAddress, Lane, Owner, Section};
use protocol::{
    CommandPayload, ControlListRequest, CounterId, CountersRequest, DecodeScratch,
    DecodedSuccessResponsePayload, DecodedTypedResponseFrame, EditPath, ExpectedRevision,
    ProtocolCodec, RequestId, SessionEdit, SessionRevision, TypedCommandFrame,
};
use session::{
    DualMonoFader, ParameterChannel, SessionModel, StableId, Vca, canonical_session_json,
};

mod common;
use common::*;

const ROUTE_FOLLOWS_MUTE_ROW: u32 = 7;
/// The own-row ID of a strip's `effective_fader_db`, a derived row.
const EFFECTIVE_FADER_DB: u32 = 64;
/// The VCA the script adds, and the one that holds it and `v`.
const ADDED_VCA: VcaRef = VcaRef { id: "w" };
const OUTER_VCA: VcaRef = VcaRef { id: "outer" };
const MATRIX_MODE_ROW: u32 = 1;
const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

/// Which lane(s) of a dual-mono value an edit writes.
#[derive(Clone, Copy)]
enum Lanes {
    Left,
    Right,
    Both,
}

impl Lanes {
    fn lane(self) -> Lane {
        match self {
            Self::Left => ParameterChannel::Left,
            Self::Right => ParameterChannel::Right,
            Self::Both => ParameterChannel::Both,
        }
    }
}

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
    /// A block-rate effect parameter.
    Parameter {
        insert: Insert,
        parameter: u32,
        lanes: Lanes,
        value: f32,
    },
    /// A parametric EQ parameter on both lanes.
    EqParameter {
        parameter: u32,
        value: f32,
    },
    Bypass {
        insert: Insert,
        bypassed: bool,
    },
    /// A parameter of a console slot on `t1` (batch 7 review M6).
    ConsoleParameter {
        slot: &'static str,
        parameter: u32,
        lanes: Lanes,
        value: f32,
    },
    /// A console slot's bypass on `t1`.
    ConsoleBypass {
        slot: &'static str,
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
    /// The send's `follows_mute` (a route's own row).
    FollowsMute {
        follows: bool,
    },
    /// A strip's `matrix_mode` (a strip's own row): `true` is the matrix, `false` the pan.
    MatrixMode {
        strip: Strip,
        matrix: bool,
    },
    /// The VCA's members become `t2` and `t3` (a VCA member added), through `UpsertVca`, with the
    /// offset and mute the VCA holds at that block.
    VcaGainsMember,
    /// One lane of a strip's high-pass.
    HpfLane {
        strip: Strip,
        lanes: Lanes,
        hz: f32,
    },
    /// A strip's `matrix_mode` on its left lane: a shared row has no lanes (`control.lane`).
    MatrixModeLane {
        strip: Strip,
    },
    /// A write to a derived row, the strip's effective fader (`control.read_only`).
    EffectiveFader {
        strip: Strip,
        db: f32,
    },
    /// Two fader lanes in one transaction with two different ramp lengths, which share one ramp
    /// row (`control.ramp.conflict`).
    FaderLaneRamps {
        strip: Strip,
    },
    /// A VCA added (`UpsertVca`) that holds `t3`.
    AddVca,
    /// A VCA that holds the VCA `v` and the added VCA `w`: a nested VCA.
    NestedVca,
}

/// How a scripted edit must end on both hosts.
#[derive(Clone, Copy, PartialEq)]
enum Outcome {
    /// Accepted: `live` on both hosts (`monitor` for solo).
    Accepted,
    /// Accepted and a no-op: the edit repeats the committed value, so both hosts answer
    /// `model_only` and write no record.
    Unchanged,
    /// Refused: the control code, on both hosts.
    Refused { code: &'static str },
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

const fn refused(block: u64, edit: Edit, code: &'static str) -> Step {
    Step {
        block,
        edit,
        ramp: None,
        outcome: Outcome::Refused { code },
    }
}

const fn refused_ramped(block: u64, edit: Edit, ramp: u32, code: &'static str) -> Step {
    Step {
        block,
        edit,
        ramp: Some(ramp),
        outcome: Outcome::Refused { code },
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
        // Console slots, one in each section: a block-rate parameter of each, then a bypass of
        // each. Their cells are the inserts' (a console slot is an effect of its strip).
        live(
            33,
            Edit::ConsoleParameter {
                slot: PRE_SLOT,
                parameter: 1,
                lanes: Lanes::Left,
                value: -30.0,
            },
        ),
        live(
            34,
            Edit::ConsoleParameter {
                slot: POST_SLOT,
                parameter: 1,
                lanes: Lanes::Both,
                value: 12.0,
            },
        ),
        live(
            35,
            Edit::ConsoleBypass {
                slot: PRE_SLOT,
                bypassed: true,
            },
        ),
        live(
            41,
            Edit::ConsoleBypass {
                slot: POST_SLOT,
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
        // Unmuted after the solo is off: while it is on, the solo mutes `t1` anyway, and the unmute
        // would change no effective value (`model_only`).
        live(
            57,
            Edit::Mute {
                strip: T1,
                lanes: Lanes::Both,
                muted: false,
            },
        ),
        // The send's `follows_mute`, with `t1` muted: following closes the send, and not following
        // lets it through. Then a strip's pan or matrix, and a member added to the VCA.
        live(
            45,
            Edit::Mute {
                strip: T1,
                lanes: Lanes::Both,
                muted: true,
            },
        ),
        live(47, Edit::FollowsMute { follows: false }),
        live(52, Edit::FollowsMute { follows: true }),
        // A strip's matrix to pan is a live write; back to the matrix keeps the effective 2x2 the
        // plan already holds, so the core writes no record (`model_only`) while the committed
        // documents differ. Both hosts must say the same.
        live(
            36,
            Edit::MatrixMode {
                strip: T3,
                matrix: false,
            },
        ),
        unchanged(
            39,
            Edit::MatrixMode {
                strip: T3,
                matrix: true,
            },
            480,
        ),
        live(56, Edit::VcaGainsMember),
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
        // A submix's trim and pan (switched from its matrix first), one lane of a high-pass. A
        // route into the output has no live lane: its gain and mute are a plan rebuild, which the
        // C ABI applies and the browser refuses `web.apply.rebuild_pending` until #1562b (D9), so
        // its rows join this script with that issue.
        live(
            48,
            Edit::Trim {
                strip: BUS,
                lanes: Lanes::Both,
                db: -3.0,
            },
        ),
        live(
            49,
            Edit::MatrixMode {
                strip: BUS,
                matrix: false,
            },
        ),
        live(
            50,
            Edit::Pan {
                strip: BUS,
                left: -0.3,
                right: 0.2,
            },
        ),
        live(
            23,
            Edit::HpfLane {
                strip: T1,
                lanes: Lanes::Left,
                hz: 120.0,
            },
        ),
        // A VCA added, then a VCA that holds it and the first one (nested), then a move of the
        // outer VCA, which reaches the members of both.
        live(51, Edit::AddVca),
        live(53, Edit::NestedVca),
        live(
            54,
            Edit::VcaOffset {
                vca: OUTER_VCA,
                lanes: Lanes::Both,
                db: -4.0,
            },
        ),
        // Refusals: one each, in the order the table in the doc names them.
        refused(
            58,
            Edit::Fader {
                strip: T1,
                lanes: Lanes::Both,
                db: 30.0,
            },
            "control.domain",
        ),
        refused(
            59,
            Edit::Solo {
                strip: BUS,
                on: true,
            },
            "control.address.unknown",
        ),
        refused(
            60,
            Edit::VcaOffset {
                vca: NO_VCA,
                lanes: Lanes::Both,
                db: -3.0,
            },
            "control.address.unknown",
        ),
        refused(61, Edit::MatrixModeLane { strip: T3 }, "control.lane"),
        // `t1` is in pan mode: a matrix row is inactive until the mode changes.
        refused(
            61,
            Edit::Matrix {
                strip: T1,
                values: [0.7, 0.3, -0.2, 0.9],
            },
            "control.inactive",
        ),
        refused(
            62,
            Edit::EffectiveFader {
                strip: T1,
                db: -3.0,
            },
            "control.read_only",
        ),
        // An explicit ramp on a row whose ramp is fixed (an effect parameter).
        refused_ramped(
            62,
            Edit::Parameter {
                insert: GATE,
                parameter: 1,
                lanes: Lanes::Both,
                value: -20.0,
            },
            480,
            "control.ramp.fixed",
        ),
        refused(
            63,
            Edit::FaderLaneRamps { strip: T2 },
            "control.ramp.conflict",
        ),
    ]
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
        maximum_vcas: 256,
        reserved: [0; 2],
    }
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
            Edit::ConsoleParameter {
                slot,
                parameter,
                lanes,
                value,
            } => vec![set(
                address(
                    Owner::Strip(id(T1.id)),
                    Section::Console(id(slot)),
                    parameter,
                    lanes.lane(),
                ),
                value,
            )],
            Edit::ConsoleBypass { slot, bypassed } => vec![set(
                address(
                    Owner::Strip(id(T1.id)),
                    Section::Console(id(slot)),
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
            Edit::FollowsMute { follows } => vec![set(
                address(
                    route(),
                    Section::Own,
                    ROUTE_FOLLOWS_MUTE_ROW,
                    ParameterChannel::Both,
                ),
                toggle(follows),
            )],
            Edit::MatrixMode { strip, matrix } => vec![set(
                address(
                    Owner::Strip(id(strip.id)),
                    Section::Own,
                    MATRIX_MODE_ROW,
                    ParameterChannel::Both,
                ),
                toggle(matrix),
            )],
            Edit::VcaGainsMember => vec![SessionEdit::UpsertVca {
                vca: Vca {
                    id: id(VCA.id),
                    fader: DualMonoFader {
                        left_db: -3.0,
                        right_db: -3.0,
                        left_mute: false,
                        right_mute: false,
                    },
                    members: vec![id(T2.id), id(T3.id)],
                },
            }],
            Edit::HpfLane { strip, lanes, hz } => {
                vec![set(Self::strip_address(strip, HPF_HZ, lanes), hz)]
            }
            Edit::MatrixModeLane { strip } => vec![set(
                address(
                    Owner::Strip(id(strip.id)),
                    Section::Own,
                    MATRIX_MODE_ROW,
                    ParameterChannel::Left,
                ),
                1.0,
            )],
            Edit::EffectiveFader { strip, db } => vec![set(
                address(
                    Owner::Strip(id(strip.id)),
                    Section::Own,
                    EFFECTIVE_FADER_DB,
                    ParameterChannel::Left,
                ),
                db,
            )],
            Edit::FaderLaneRamps { strip } => [(Lanes::Left, 64), (Lanes::Right, 480)]
                .into_iter()
                .map(|(lanes, length)| SessionEdit::SetControl {
                    address: Self::strip_address(strip, FADER_DB, lanes),
                    value: -5.0,
                    ramp_samples: Some(length),
                })
                .collect(),
            Edit::AddVca => vec![SessionEdit::UpsertVca {
                vca: Vca {
                    id: id(ADDED_VCA.id),
                    fader: DualMonoFader {
                        left_db: -2.0,
                        right_db: -2.0,
                        left_mute: false,
                        right_mute: false,
                    },
                    members: vec![id(T3.id)],
                },
            }],
            Edit::NestedVca => vec![SessionEdit::UpsertVca {
                vca: Vca {
                    id: id(OUTER_VCA.id),
                    fader: DualMonoFader {
                        left_db: -1.0,
                        right_db: -1.0,
                        left_mute: false,
                        right_mute: false,
                    },
                    members: vec![id(VCA.id), id(ADDED_VCA.id)],
                },
            }],
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

    /// One scripted edit as one transaction at the committed revision, and the reply frame's bytes.
    fn edit(&mut self, edit: Edit, ramp: Option<u32>) -> (Verdict, Vec<u8>) {
        let edits = Self::edits(edit, ramp);
        let response = self.send(
            CommandPayload::SessionTransactionApply(&edits),
            ExpectedRevision::Exact(SessionRevision(self.revision)),
        );
        let verdict = decode_verdict(&response);
        if let Verdict::Applied { revision, .. } = &verdict {
            self.revision = *revision;
        }
        (verdict, response)
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

    /// The plan's applied-revision watermark through `miso_engine_v1_plan_watermark`: `(revision,
    /// first sample, outcome flags)`.
    fn watermark(&self) -> (u64, u64, u64) {
        let mut out = capi::Watermark {
            struct_size: capi::WATERMARK_SIZE,
            reserved0: 0,
            revision: 0,
            first_sample: 0,
            outcome_flags: 0,
            exact_count: 0,
            transition_fallback_count: 0,
            superseded_count: 0,
            reserved: [0; 5],
        };
        // SAFETY: The plan is live; the query is pure and `out` is owned storage of the exact size.
        let code = unsafe { capi::miso_engine_v1_plan_watermark(self.plan, &mut out) };
        assert_eq!(code, C_RESULT_OK, "C ABI watermark");
        (out.revision, out.first_sample, out.outcome_flags)
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

/// Every page of `CONTROL_LIST_GET` and `CONTROL_STATE_GET` and the `SESSION_SNAPSHOT_GET` bytes
/// through one host's command entry, as text and bytes that two hosts can compare.
fn dump(
    mut send: impl FnMut(CommandPayload<'_>, ExpectedRevision) -> Vec<u8>,
) -> (Vec<String>, Vec<u8>) {
    let mut rows = Vec::new();
    for state in [false, true] {
        let mut epoch = 0;
        let mut after = 0;
        loop {
            let request = ControlListRequest {
                structure_epoch: epoch,
                after_handle: after,
                limit: 256,
                owner: None,
            };
            let bytes = send(
                if state {
                    CommandPayload::ControlStateGet(&request)
                } else {
                    CommandPayload::ControlListGet(&request)
                },
                ExpectedRevision::Any,
            );
            let mut fields = [0_u16; 4096];
            let DecodedTypedResponseFrame::Success { payload, .. } = ProtocolCodec::default()
                .decode_typed_response(&bytes, &mut DecodeScratch::new(&mut fields))
                .expect("a control page")
            else {
                panic!("expected a control page")
            };
            // The pages differ in size between the hosts (the C ABI leg's frame is smaller), so
            // the rows are compared one by one, the page header once.
            let (eof, last, page_epoch) = match payload {
                DecodedSuccessResponsePayload::ControlListPage(page) => {
                    if after == 0 {
                        rows.push(format!("list r{} e{}", page.revision, page.structure_epoch));
                    }
                    rows.extend(page.entries.iter().map(|entry| format!("{entry:?}")));
                    (page.eof, page.last_handle, page.structure_epoch)
                }
                DecodedSuccessResponsePayload::ControlStatePage(page) => {
                    if after == 0 {
                        rows.push(format!(
                            "state r{} e{}",
                            page.revision, page.structure_epoch
                        ));
                    }
                    rows.extend(page.records.iter().map(|record| format!("{record:?}")));
                    (page.eof, page.last_handle, page.structure_epoch)
                }
                _ => panic!("expected a control page"),
            };
            if eof {
                break;
            }
            epoch = page_epoch;
            after = last;
        }
    }
    let snapshot = snapshot_bytes(send);
    (rows, snapshot)
}

/// One run of `steps` on both legs. Returns the browser's blocks and the final superseded count
/// (equal on both legs, asserted here).
fn run(rate: u32, steps: &[Step]) -> (Vec<Vec<u32>>, u64) {
    let model = model_at(rate);
    let mut browser = Rig::new(&model);
    let mut c_abi = CAbi::new(&model);
    let mut blocks = Vec::new();
    for block in 0..BLOCKS {
        for (position, step) in steps.iter().enumerate().filter(|(_, s)| s.block == block) {
            let label = format!("{rate} Hz: step {position} before block {block}");
            let (b, b_reply) = browser.transaction_reply(&CAbi::edits(step.edit, step.ramp));
            let (c, c_reply) = c_abi.edit(step.edit, step.ramp);
            assert_eq!(b, c, "{label}: the two hosts give one verdict");
            assert_eq!(
                b_reply, c_reply,
                "{label}: the two hosts give the same reply bytes"
            );
            let solo = matches!(step.edit, Edit::Solo { .. });
            match step.outcome {
                Outcome::Accepted => {
                    let path = if solo {
                        EditPath::Monitor
                    } else {
                        EditPath::Live
                    };
                    assert!(
                        matches!(&c, Verdict::Applied { path: got, .. } if *got == path),
                        "{label}: the path is {path:?}, got {c:?}"
                    );
                }
                Outcome::Unchanged => {
                    let path = if solo {
                        EditPath::Monitor
                    } else {
                        EditPath::ModelOnly
                    };
                    assert!(
                        matches!(&c, Verdict::Applied { path: got, .. } if *got == path),
                        "{label}: the path is {path:?}, got {c:?}"
                    );
                }
                Outcome::Refused { code } => {
                    assert!(
                        matches!(&c, Verdict::Refused { codes, .. } if codes.iter().any(|c| c == code)),
                        "{label}: refused with {code}, got {c:?}"
                    );
                }
            }
        }
        let (b, c) = (browser.step(block), c_abi.step(block));
        // #1349 gate 1: the status words are the C ABI's watermark after every block.
        assert_eq!(
            status_watermark(&browser),
            c_abi.watermark(),
            "{rate} Hz: block {block}: the browser status carries the C ABI's watermark"
        );
        let differing = b.iter().zip(&c).filter(|(x, y)| x != y).count();
        assert_eq!(
            differing, 0,
            "{rate} Hz: block {block} differs between the browser and the C ABI in {differing} samples"
        );
        blocks.push(b);
    }
    let superseded = c_abi.superseded();
    assert_eq!(
        browser.host.status().live_values_superseded,
        superseded,
        "{rate} Hz: the browser status reports the superseded count the C ABI reports"
    );
    // Gate 8: the control list, the control state and the committed document are the same bytes.
    let (browser_pages, browser_snapshot) =
        dump(|payload, expected| browser.command(payload, expected));
    let (c_pages, c_snapshot) = dump(|payload, expected| c_abi.send(payload, expected));
    assert_eq!(
        browser_pages.len(),
        c_pages.len(),
        "{rate} Hz: the same number of control rows"
    );
    for (index, (b, c)) in browser_pages.iter().zip(&c_pages).enumerate() {
        assert_eq!(b, c, "{rate} Hz: control row {index} differs");
    }
    assert_eq!(
        browser_snapshot, c_snapshot,
        "{rate} Hz: the committed documents are byte-identical"
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

/// The three watermark words of the browser's status: `(revision, first sample, outcome flags)`.
fn status_watermark(browser: &Rig) -> (u64, u64, u64) {
    let status = browser.host.status();
    (
        status.watermark_revision,
        status.watermark_first_sample,
        status.watermark_outcome_flags,
    )
}

/// #1349 gates 1 and 3: the status words stay at the initial revision until the block that applies
/// a live edit, then read `(r2, that block's first sample, EXACT)`. That is the C ABI's watermark
/// for the same script and the record `read_watermark` copies from the plan's query.
///
/// Test value: red if the browser computes its own application sample (it would report the new
/// revision before block 3, or block 2's sample), if the status copies from a different advance
/// than the record published (a block late), or if a block that advanced nothing rewrites the words.
#[test]
fn the_status_words_carry_the_applied_revision_the_c_abi_reports() {
    for rate in RATES {
        let model = model_at(rate);
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        let initial = (model.revision, 0, 1);
        assert_eq!(status_watermark(&browser), initial, "{rate} Hz: boot");
        assert_eq!(c_abi.watermark(), initial, "{rate} Hz: C ABI boot");

        let edit = Edit::Fader {
            strip: T1,
            lanes: Lanes::Both,
            db: -6.0,
        };
        for block in 0..3 {
            let _ = (browser.step(block), c_abi.step(block));
            assert_eq!(
                status_watermark(&browser),
                initial,
                "{rate} Hz: block {block}"
            );
            assert_eq!(c_abi.watermark(), initial);
        }
        // Committed before block 3: pending until the block that applies it renders.
        let (verdict, _) = browser.transaction_reply(&CAbi::edits(edit, None));
        let (c_verdict, _) = c_abi.edit(edit, None);
        assert_eq!(verdict, c_verdict);
        let Verdict::Applied { revision, .. } = verdict else {
            panic!("the live edit commits")
        };
        assert_eq!(revision, model.revision + 1);
        assert_eq!(
            status_watermark(&browser),
            initial,
            "{rate} Hz: never early"
        );

        let _ = (browser.step(3), c_abi.step(3));
        let applied = (revision, 3 * u64::from(QUANTUM), 1);
        assert_eq!(status_watermark(&browser), applied, "{rate} Hz: block 3");
        assert_eq!(c_abi.watermark(), applied, "{rate} Hz: C ABI block 3");
        // Gate 3: the control-side record is the same level.
        assert_eq!(browser.host.read_watermark(), host_web::RESULT_OK);
        let record = browser.host.watermark();
        assert_eq!(
            (record.revision, record.first_sample, record.outcome_flags),
            applied
        );

        // A block that advances nothing leaves the words as they were.
        let _ = (browser.step(4), c_abi.step(4));
        assert_eq!(status_watermark(&browser), applied, "{rate} Hz: block 4");
        assert_eq!(c_abi.watermark(), applied);
    }
}

/// The browser leg alone over `steps`: its blocks. The audibility check below compares scripts
/// that differ by one edit, which the C ABI would answer `model_only` when the edit left out was
/// the one that made a later edit a change.
fn browser_blocks(rate: u32, steps: &[Step]) -> Vec<Vec<u32>> {
    let mut browser = Rig::new(&model_at(rate));
    (0..BLOCKS)
        .map(|block| {
            for step in steps.iter().filter(|step| step.block == block) {
                browser.transaction(&CAbi::edits(step.edit, step.ramp));
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

// ---------------------------------------------------------------------------------------------
// The VCA bounds (batch 7 review M1, ruling V-Q3)
// ---------------------------------------------------------------------------------------------

/// `tracks` tracks of one source under a chain of `vcas` VCAs: `v000` holds every track and each
/// later VCA holds the one before, so every track is reached by every VCA (`tracks * vcas` pairs).
fn bound_model(tracks: usize, vcas: usize) -> SessionModel {
    let mut model = model_at(48_000);
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    model.submixes.clear();
    model.vcas.clear();
    let source = model.sources[0].clone();
    let template = model.tracks[0].clone();
    let route = model.routes[0].clone();
    model.sources = vec![source.clone()];
    model.tracks.clear();
    model.routes.clear();
    for index in 0..tracks {
        let mut track = template.clone();
        track.id = id(&format!("x{index:03}"));
        track.source_id = source.id.clone();
        track.console.clear();
        track.inserts.effects.clear();
        let mut route = route.clone();
        route.id = id(&format!("x{index:03}-main"));
        route.source = session::RouteSource::Track {
            track_id: track.id.clone(),
            tap: session::SendTap::PostPan,
        };
        model.tracks.push(track);
        model.routes.push(route);
    }
    for index in 0..vcas {
        let members = if index == 0 {
            model.tracks.iter().map(|track| track.id.clone()).collect()
        } else {
            vec![id(&format!("v{:03}", index - 1))]
        };
        model.vcas.push(Vca {
            id: id(&format!("v{index:03}")),
            fader: DualMonoFader {
                left_db: 0.0,
                right_db: 0.0,
                left_mute: false,
                right_mute: false,
            },
            members,
        });
    }
    model
}

/// What the C ABI answers a compile of `model` with: its result code and its diagnostics. A
/// session that compiles is destroyed again.
fn c_abi_compile(model: &SessionModel) -> (u32, Vec<u8>) {
    let document = canonical_session_json(model).expect("canonical session");
    let config = EngineConfig {
        struct_size: ENGINE_CONFIG_SIZE,
        abi_version: C_ABI_VERSION,
        reserved: [0; 4],
    };
    let (mut engine, mut session, mut plan) = (ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
    let mut storage = [0_u8; 4_096];
    let mut diagnostics = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: storage.as_mut_ptr(),
        capacity_bytes: storage.len() as u64,
        required_bytes: 0,
    };
    // SAFETY: Every descriptor and output location lives for the complete call; the handles a
    // successful compile returns are destroyed once.
    unsafe {
        assert_eq!(
            capi::miso_engine_v1_engine_create(&config, &mut engine),
            C_RESULT_OK
        );
        let code = capi::miso_engine_v1_compile_session(
            engine,
            document.as_ptr(),
            document.len() as u64,
            &limits(),
            &mut diagnostics,
            &mut session,
            &mut plan,
        );
        capi::miso_engine_v1_engine_destroy(engine);
        if code == C_RESULT_OK {
            capi::miso_engine_v1_session_destroy(session);
            capi::miso_engine_v1_plan_destroy(plan);
        }
        (
            code,
            storage[..diagnostics.required_bytes as usize].to_vec(),
        )
    }
}

/// The browser's diagnostics for a boot of `model`: `None` when it boots.
fn browser_boot_refusal(model: &SessionModel) -> Option<Vec<u8>> {
    let document = canonical_session_json(model).expect("canonical session");
    let options = host_web::WebBootOptions {
        require_sample_rate_hz: model.sample_rate_hz,
        require_quantum_frames: QUANTUM,
        source_ring_frames: QUANTUM * 2,
        ..host_web::WebBootOptions::explicit_defaults()
    };
    host_web::AudioWorkletEngineHost::boot(document.as_bytes(), options)
        .err()
        .map(|failure| failure.diagnostic().to_vec())
}

/// Both hosts take a session at the VCA bound (256 VCAs, 16,384 reach pairs) and refuse one past
/// either bound with the same diagnostic: the core's limits, not a rule of one host.
///
/// Test value: red if one host enforces the bound and the other does not (the browser composes a
/// VCA move on the worklet thread, the C ABI on the host's control thread; both run the same core),
/// if the reach pairs go uncounted so that 65 tracks under 256 nested VCAs boot, or if either
/// host's bound drifts from `host_core::MAXIMUM_VCA_REACH_PAIRS` and `MAXIMUM_VCAS_BOUNDED`.
#[test]
fn both_hosts_bound_the_vcas_alike() {
    // 64 tracks x 256 VCAs is exactly 16,384 pairs and 256 VCAs: both boot.
    let at_the_bound = bound_model(64, 256);
    assert_eq!(
        at_the_bound.vca_reach_pairs(u64::MAX),
        host_core::MAXIMUM_VCA_REACH_PAIRS
    );
    assert_eq!(c_abi_compile(&at_the_bound).0, C_RESULT_OK);
    assert_eq!(browser_boot_refusal(&at_the_bound), None);

    // One track more is 16,640 pairs.
    let too_many_pairs = bound_model(65, 256);
    let (code, c_diagnostics) = c_abi_compile(&too_many_pairs);
    assert_ne!(code, C_RESULT_OK);
    let browser_diagnostics = browser_boot_refusal(&too_many_pairs).expect("the browser refuses");
    assert_eq!(c_diagnostics, browser_diagnostics, "reach pairs");
    assert_eq!(
        c_diagnostics,
        b"host.resource.vca_reach_pairs\t$\n".to_vec(),
        "the core's own diagnostic"
    );

    // 257 VCAs (no members: no pairs) is one over the count.
    let too_many_vcas = bound_model(2, 257);
    let (code, c_diagnostics) = c_abi_compile(&too_many_vcas);
    assert_ne!(code, C_RESULT_OK);
    let browser_diagnostics = browser_boot_refusal(&too_many_vcas).expect("the browser refuses");
    assert_eq!(c_diagnostics, browser_diagnostics, "VCA count");
    assert_eq!(c_diagnostics, b"host.resource.count\t$\n".to_vec());
}
