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
//! repeats at 44.1, 48, 88.2 and 96 kHz. The script holds the edits that take the rebuild arm (a
//! route into the output, the prepared bypass of the delay and the multiband compressor, and
//! `delay_samples`): the browser prepares and publishes the successor like the C ABI does, and the
//! two hosts answer the same bytes and render the same bits across the swap (#1562b). No launch
//! effect declares an audio-rate parameter, so that row of the batch 7 amendment has no edit to
//! script.
//!
//! It absorbs #1364 gate 3 (the edits carry no per-edit ramp, so each host resolves the session's
//! own length, and one pan carries a window of its own while another does not) and #1399 (a fader,
//! a pan and an EQ parameter drain alike, with equal superseded counts), whose two narrow tests it
//! replaces.
//!
//! Bypass: both hosts crossfade a live bypass of an insert and of a console slot over the session's
//! `mute_ms` ramp (#1341a1, #1341a2), because the browser is a thin client of the core's
//! `SessionState`. The scripted bypass edits of the main test, the insert's and both console
//! slots' (a console slot always banks), are compared bit for bit like every other row, and
//! `a_live_bypass_crossfades_alike_on_both_hosts` proves the insert row is a crossfade (neither a
//! step nor a no-op) at every launch rate. This session's strips hold effects after the console
//! slot, whose own history outlasts the ramp, so the console slot's shape (its ramp, its settle on
//! the booted document, and that a bank renders what a node renders) is gated where nothing
//! follows it: `sdk/test/live-controls-evals.mjs` and `graph-compiler`'s `bypass_shunt_identity`.
//! A parity defect at the bypass row is never weakened away.

#![allow(unsafe_code)]

use core::ptr;

use capi::{
    ABI_VERSION as C_ABI_VERSION, BYTES_OUT_SIZE, BytesOut, COMPILE_LIMITS_SIZE, CompileLimits,
    ENGINE_CONFIG_SIZE, EVENT_LANE_RELIABLE, EngineConfig, PLAN_RESOURCE_REPORT_SIZE,
    PLANAR_OUTPUT_SIZE, Plan, PlanResourceReport, PlanarOutput, RESULT_OK as C_RESULT_OK,
    SOURCE_CHUNK_SIZE, SOURCE_SEEK_REPORT_SIZE, SUBMIT_REPORT_SIZE, Session, SourceChunk,
    SourceSeekReport, SubmitReport,
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
/// The builtin row of a strip's `delay_samples`.
const DELAY_SAMPLES_ROW: u32 = 11;
/// The builtin rows of a strip's `pan` and `matrix_ll`.
const PAN_ROW: u32 = 12;
const MATRIX_LL_ROW: u32 = 7;
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
    /// The gain of the route from `t1` into the output (a rebuild: that route has no live lane).
    OutputGain {
        db: f32,
    },
    /// The mute of the route from `t1` into the output (a rebuild).
    OutputMute {
        muted: bool,
    },
    /// A strip's `delay_samples` on both lanes (a rebuild: it moves the plan's latency).
    DelaySamples {
        strip: Strip,
        samples: f32,
    },
}

/// How a scripted edit must end on both hosts.
#[derive(Clone, Copy, PartialEq)]
enum Outcome {
    /// Accepted: `live` on both hosts (`monitor` for solo).
    Accepted,
    /// Accepted by a plan rebuild: `rebuild` on both hosts (#1562b).
    Rebuilt,
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

const fn rebuilt(block: u64, edit: Edit) -> Step {
    Step {
        block,
        edit,
        ramp: None,
        outcome: Outcome::Rebuilt,
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
        // The rebuild arm (#1562b): a route into the output has no live lane, so its gain and
        // mute rebuild the plan; so does the prepared bypass of the delay and of the multiband
        // compressor, and a strip's `delay_samples`, which moves the plan's latency. Each is a
        // swap at the next block on both hosts, with the watermark advancing at that block.
        rebuilt(11, Edit::OutputGain { db: -3.0 }),
        rebuilt(17, Edit::OutputMute { muted: true }),
        rebuilt(19, Edit::OutputMute { muted: false }),
        rebuilt(
            21,
            Edit::DelaySamples {
                strip: T1,
                samples: 64.0,
            },
        ),
        rebuilt(
            34,
            Edit::Bypass {
                insert: MULTIBAND,
                bypassed: true,
            },
        ),
        rebuilt(
            43,
            Edit::Bypass {
                insert: DELAY,
                bypassed: true,
            },
        ),
        // A submix's trim and pan (switched from its matrix first), one lane of a high-pass.
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
            expected_effect: None,
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
                    expected_effect: None,
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
            Edit::OutputGain { db } => vec![set(
                address(
                    Owner::Route(id("t1-main")),
                    Section::Own,
                    ROUTE_GAIN_DB,
                    ParameterChannel::Both,
                ),
                db,
            )],
            Edit::OutputMute { muted } => vec![set(
                address(
                    Owner::Route(id("t1-main")),
                    Section::Own,
                    ROUTE_MUTE,
                    ParameterChannel::Both,
                ),
                toggle(muted),
            )],
            Edit::DelaySamples { strip, samples } => vec![set(
                Self::strip_address(strip, DELAY_SAMPLES_ROW, Lanes::Both),
                samples,
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
        let (code, response) = self.send_status(payload, expected);
        assert_eq!(code, C_RESULT_OK, "C ABI command");
        response
    }

    /// The session's last diagnostic, through `miso_engine_v1_last_error`.
    fn last_error(&self) -> Vec<u8> {
        let mut storage = vec![0_u8; 4_096];
        let mut out = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: storage.as_mut_ptr(),
            capacity_bytes: storage.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: The session is live and used by this thread alone; the buffer is owned.
        let code = unsafe { capi::miso_engine_v1_last_error(self.session.cast(), &mut out) };
        assert_eq!(code, C_RESULT_OK, "C ABI last error");
        storage.truncate(out.required_bytes as usize);
        storage
    }

    /// [`Self::send`] for a command that may be refused: the entry's result, and the response frame
    /// after `RESULT_OK` or else the refusal's diagnostic.
    fn send_status(
        &mut self,
        payload: CommandPayload<'_>,
        expected: ExpectedRevision,
    ) -> (u32, Vec<u8>) {
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
        self.send_raw(&bytes)
    }

    /// Submit one command frame as given, and answer as [`Self::send_status`] does.
    fn send_raw(&mut self, bytes: &[u8]) -> (u32, Vec<u8>) {
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
        response.truncate(out.required_bytes as usize);
        // Before the drain: a later entry clears the diagnostic.
        if code != C_RESULT_OK {
            response = self.last_error();
        }
        self.drain_reliable();
        (code, response)
    }

    /// One scripted edit as one transaction at the committed revision, and the reply frame's bytes.
    fn edit(&mut self, edit: Edit, ramp: Option<u32>) -> (Verdict, Vec<u8>) {
        self.transaction(&Self::edits(edit, ramp))
    }

    /// `edits` as one transaction at the committed revision, and the reply frame's bytes.
    fn transaction(&mut self, edits: &[SessionEdit]) -> (Verdict, Vec<u8>) {
        let response = self.send(
            CommandPayload::SessionTransactionApply(edits),
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

    /// The active plan's `(latency_samples, tail_kind, tail_samples)` through
    /// `miso_engine_v1_plan_resources`.
    fn plan_latency(&self) -> (u64, u64, u64) {
        // SAFETY: `PlanResourceReport` is plain integers, for which all-zero bytes are a value.
        let mut out: PlanResourceReport = unsafe { core::mem::zeroed() };
        out.struct_size = PLAN_RESOURCE_REPORT_SIZE;
        // SAFETY: The plan is live; the query is pure and `out` is owned storage of the exact size.
        let code = unsafe { capi::miso_engine_v1_plan_resources(self.plan, &mut out) };
        assert_eq!(code, C_RESULT_OK, "C ABI plan resources");
        (out.latency_samples, out.tail_kind, out.tail_samples)
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
            self.submit(&format!("src-{}", strip.id), feed as u64, block);
        }
        self.render(block)
    }

    /// Submit block `block`'s PCM of feed `feed` to the source `name`, at the source's own position
    /// `block`.
    fn submit(&mut self, name: &str, feed: u64, block: u64) {
        self.submit_from(name, feed, block, block, 1);
    }

    /// [`Self::submit`], with the chunk of `generation` starting at the source's frame
    /// `start_block * QUANTUM`.
    fn submit_from(
        &mut self,
        name: &str,
        feed: u64,
        block: u64,
        start_block: u64,
        generation: u64,
    ) {
        let [left, right] = planes(feed, block);
        let pointers = [left.as_ptr(), right.as_ptr()];
        let chunk = SourceChunk {
            struct_size: SOURCE_CHUNK_SIZE,
            sample_rate_hz: self.rate,
            generation,
            start_frame: start_block * u64::from(QUANTUM),
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
        assert_eq!(code, C_RESULT_OK, "C ABI source {name}");
        assert_eq!(report.accepted_frames, u64::from(QUANTUM));
    }

    /// Render block `block` and return the output's bits.
    fn render(&mut self, block: u64) -> Vec<u32> {
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

/// The owners of the control list's `master` rows that are on (#1683), read by the rows' typed
/// fields.
fn listed_masters(
    mut send: impl FnMut(CommandPayload<'_>, ExpectedRevision) -> Vec<u8>,
) -> Vec<String> {
    let mut masters = Vec::new();
    let (mut epoch, mut after) = (0, 0);
    loop {
        let request = ControlListRequest {
            structure_epoch: epoch,
            after_handle: after,
            limit: 256,
            owner: None,
        };
        let bytes = send(
            CommandPayload::ControlListGet(&request),
            ExpectedRevision::Any,
        );
        let mut fields = [0_u16; 4096];
        let DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::ControlListPage(page),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&bytes, &mut DecodeScratch::new(&mut fields))
            .expect("a control page")
        else {
            panic!("expected a control list page")
        };
        masters.extend(page.entries.iter().filter_map(|entry| match &entry.body {
            protocol::ControlBody::Control(row) if row.name == "master" && row.value == 1.0 => {
                entry.address.owner_id.clone()
            }
            _ => None,
        }));
        if page.eof {
            return masters;
        }
        epoch = page.structure_epoch;
        after = page.last_handle;
    }
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
                Outcome::Rebuilt => {
                    assert!(
                        matches!(
                            &c,
                            Verdict::Applied {
                                path: EditPath::Rebuild,
                                ..
                            }
                        ),
                        "{label}: the path is rebuild, got {c:?}"
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
        // #1688 D4: the browser's latency and tail words are the C ABI report's, after every
        // scripted edit (the plan that renders has not changed yet) and after every block.
        let latency = c_abi.plan_latency();
        assert_eq!(
            browser_plan_latency(&mut browser),
            latency,
            "{rate} Hz: before block {block}: the browser plan latency is the C ABI's"
        );
        let (b, c) = (browser.step(block), c_abi.step(block));
        assert_eq!(
            browser_plan_latency(&mut browser),
            c_abi.plan_latency(),
            "{rate} Hz: after block {block}: the browser plan latency is the C ABI's"
        );
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

/// The browser record's `(latency_samples, tail_kind, tail_samples)` after a fresh read (#1688).
fn browser_plan_latency(browser: &mut Rig) -> (u64, u64, u64) {
    assert_eq!(
        browser.host.read_plan_latency(),
        host_web::RESULT_OK,
        "the browser plan latency read"
    );
    let record = browser.host.plan_latency();
    (
        record.latency_samples,
        record.tail_kind,
        record.tail_samples,
    )
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

/// #1349's rebuild leg (#1562b): a rebuild, then a live edit before the swap. The status words
/// stay at the initial revision through the blocks before the swap block (neither commit is
/// applied until a plan that carries it renders), then both revisions are in effect together from
/// the swap block's first sample, and the words equal the C ABI's `miso_engine_v1_plan_watermark`
/// after every block.
///
/// Test value: red if the browser publishes a rebuild's revision when it commits rather than when
/// the swap block renders (the words would move before block 3), if the live edit committed behind
/// a pending rebuild is reported at its own earlier block, or if the swap block's sample or
/// outcome flags are the browser's own computation and not the plan's.
#[test]
fn the_status_words_carry_a_rebuilds_watermark_the_c_abi_reports() {
    for rate in RATES {
        let model = model_at(rate);
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        let initial = (model.revision, 0, 1);
        for block in 0..3 {
            let _ = (browser.step(block), c_abi.step(block));
            assert_eq!(status_watermark(&browser), initial, "{rate} Hz: {block}");
            assert_eq!(c_abi.watermark(), initial);
        }
        let rebuild = Edit::DelaySamples {
            strip: T1,
            samples: 64.0,
        };
        let live_edit = Edit::Fader {
            strip: T2,
            lanes: Lanes::Both,
            db: -6.0,
        };
        let (rebuilt, _) = browser.transaction_reply(&CAbi::edits(rebuild, None));
        let (c_rebuilt, _) = c_abi.edit(rebuild, None);
        assert_eq!(rebuilt, c_rebuilt);
        assert!(matches!(
            rebuilt,
            Verdict::Applied {
                path: EditPath::Rebuild,
                revision
            } if revision == model.revision + 1
        ));
        assert_eq!(
            status_watermark(&browser),
            initial,
            "{rate} Hz: never early"
        );
        assert_eq!(c_abi.watermark(), initial);
        // The live edit rides the pending successor's lanes.
        let (edited, _) = browser.transaction_reply(&CAbi::edits(live_edit, None));
        let (c_edited, _) = c_abi.edit(live_edit, None);
        assert_eq!(edited, c_edited);
        assert!(matches!(
            edited,
            Verdict::Applied {
                path: EditPath::Live,
                revision
            } if revision == model.revision + 2
        ));
        assert_eq!(
            status_watermark(&browser),
            initial,
            "{rate} Hz: still pending"
        );
        assert_eq!(c_abi.watermark(), initial);

        let _ = (browser.step(3), c_abi.step(3));
        assert_eq!(
            status_watermark(&browser),
            c_abi.watermark(),
            "{rate} Hz: the swap block"
        );
        let (revision, first_sample, flags) = status_watermark(&browser);
        assert_eq!(
            revision,
            model.revision + 2,
            "{rate} Hz: both revisions apply"
        );
        assert_eq!(
            first_sample,
            3 * u64::from(QUANTUM),
            "{rate} Hz: the swap block"
        );
        let _ = flags;
        assert_eq!(browser.host.read_watermark(), host_web::RESULT_OK);
        let record = browser.host.watermark();
        assert_eq!(
            (record.revision, record.first_sample, record.outcome_flags),
            c_abi.watermark(),
            "{rate} Hz: the control-side record"
        );
        for block in 4..8 {
            let _ = (browser.step(block), c_abi.step(block));
            assert_eq!(
                status_watermark(&browser),
                c_abi.watermark(),
                "{rate} Hz: block {block}"
            );
        }
    }
}

/// A reply frame with its request ID (header bytes 24 to 32) zeroed: each host numbers its own
/// requests, and nothing else of a reply depends on the count.
fn without_request_id(mut reply: Vec<u8>) -> Vec<u8> {
    reply[24..32].fill(0);
    reply
}

/// Blocks `range` on both hosts, equal bit for bit.
fn same_blocks(browser: &mut Rig, c_abi: &mut CAbi, range: std::ops::Range<u64>, label: &str) {
    for block in range {
        let (b, c) = (browser.step(block), c_abi.step(block));
        assert_eq!(b, c, "block {block} {label}");
    }
}

/// #1678: an effect-row edit that names the native effect it was written for is refused by the
/// core, with `control.effect.changed`, once a rebuild has replaced that effect, and nothing
/// changes; the same edit without the expected effect, and one that names the effect that runs now,
/// commit live as before. Both hosts answer alike: one verdict, the same reply bytes, the same
/// control rows and document bytes before and after the refusal, the same bits of every block.
///
/// Test value: red if either host skips the check (the bypass row 0 exists on every effect, so
/// the stale edit would land on the new effect's bypass and commit), if a host reports a different
/// code, or if the refused edit leaves the document or the rendered blocks different from the C ABI.
#[test]
fn a_stale_effect_row_edit_is_refused_alike_on_both_hosts() {
    let replace = SessionEdit::SetEffectIdentity {
        track_id: id(SHAPER.track.id),
        rack_name: session::RackName::Inserts,
        effect_id: id(SHAPER.id),
        identity: session::EffectIdentity::Native {
            effect_id: id(CLIP.native),
        },
    };
    let bypass = |expected: Option<&str>, bypassed: bool| {
        let edit = SessionEdit::set_control(
            address(
                Owner::Strip(id(SHAPER.track.id)),
                Section::Inserts(id(SHAPER.id)),
                controls::BYPASS_CONTROL,
                ParameterChannel::Both,
            ),
            f32::from(u8::from(bypassed)),
            None,
        );
        match expected {
            Some(effect) => edit.with_expected_effect(id(effect)),
            None => edit,
        }
    };
    for rate in RATES {
        let model = model_at(rate);
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        same_blocks(&mut browser, &mut c_abi, 0..3, "before the rebuild");
        // The rebuild replaces the transient shaper with the soft clip; the swap is block 3.
        let (b, b_reply) = browser.transaction_reply(std::slice::from_ref(&replace));
        let (c, c_reply) = c_abi.transaction(std::slice::from_ref(&replace));
        assert_eq!(b, c, "{rate} Hz: one verdict for the rebuild");
        assert_eq!(b_reply, c_reply);
        assert!(matches!(
            c,
            Verdict::Applied {
                path: EditPath::Rebuild,
                ..
            }
        ));
        same_blocks(&mut browser, &mut c_abi, 3..6, "after the swap");
        let before_b = dump(|payload, expected| browser.command(payload, expected));
        let before_c = dump(|payload, expected| c_abi.send(payload, expected));
        assert_eq!(before_b, before_c);
        let revision = browser.revision;
        // Written for the shaper, the insert runs the soft clip now.
        let stale = [bypass(Some(SHAPER.native), true)];
        let (b, b_reply) = browser.transaction_reply(&stale);
        let (c, c_reply) = c_abi.transaction(&stale);
        assert_eq!(b, c, "{rate} Hz: one verdict for the stale edit");
        assert_eq!(
            without_request_id(b_reply),
            without_request_id(c_reply),
            "{rate} Hz: one reply for the stale edit"
        );
        assert!(
            matches!(&c, Verdict::Refused { codes, revision: got }
                if codes.iter().any(|code| code == "control.effect.changed") && *got == revision),
            "{rate} Hz: refused as an effect change at the committed revision, got {c:?}"
        );
        // Nothing changed: the control rows and the document are the bytes they were.
        assert_eq!(
            dump(|payload, expected| browser.command(payload, expected)),
            before_b
        );
        assert_eq!(
            dump(|payload, expected| c_abi.send(payload, expected)),
            before_c
        );
        same_blocks(&mut browser, &mut c_abi, 6..8, "after the refusal");
        // Without an expected effect the same edit lands on whatever runs there: as before.
        for (round, (expected, bypassed)) in [(None, true), (Some(CLIP.native), false)]
            .into_iter()
            .enumerate()
        {
            let edits = [bypass(expected, bypassed)];
            let (b, b_reply) = browser.transaction_reply(&edits);
            let (c, c_reply) = c_abi.transaction(&edits);
            assert_eq!(b, c, "{rate} Hz: one verdict for {expected:?}");
            assert_eq!(without_request_id(b_reply), without_request_id(c_reply));
            assert!(
                matches!(
                    &c,
                    Verdict::Applied {
                        path: EditPath::Live,
                        ..
                    }
                ),
                "{rate} Hz: {expected:?} commits live, got {c:?}"
            );
            let first = 8 + 2 * round as u64;
            same_blocks(
                &mut browser,
                &mut c_abi,
                first..first + 2,
                "after a live edit",
            );
        }
    }
}

/// #1623 (root ruling, thick core): a `RESPONSE_GET` frame whose value the Rust encoder refuses
/// -- a NaN or infinite axis endpoint, a NaN override value, a bypass value between 0 and 1 -- is
/// decoded as sent and refused by the core on both hosts, with one result, the same reply bytes
/// and the same typed code (`response.grid`, `response.override.value`,
/// `response.override.bypass`), and changes nothing: the control rows and the document are the
/// bytes they were on each host.
///
/// Test value: red if either host's codec refuses the value again (the code becomes the generic
/// `protocol.failure`), if one host checks a value the other does not, or if a refusal changes
/// state.
#[test]
fn a_response_request_with_a_bad_value_is_refused_alike_on_both_hosts() {
    let request = protocol::ResponseGetRequest {
        target: protocol::ResponseTarget::Configuration(protocol::ResponseConfiguration::Effect {
            effect: "miso.parametric-eq".to_owned(),
            quality: session::EffectQuality::Normal,
            link_mode: session::LinkMode::DualMono,
        }),
        grid: protocol::ResponseGrid::Logarithmic,
        points: 64,
        minimum_hz: 20.0,
        maximum_hz: 20_000.0,
        channels: protocol::ParameterChannel::Both,
        sections: false,
        overrides: vec![
            protocol::ResponseOverride {
                control: protocol::RESPONSE_BYPASS_CONTROL,
                lane: protocol::ParameterChannel::Both,
                value: 1.0,
            },
            protocol::ResponseOverride {
                control: 4,
                lane: protocol::ParameterChannel::Both,
                value: 3.25,
            },
        ],
        configuration_id: None,
        sample_rate_hz: None,
        quantum_frames: None,
        maximum_prepared_bytes: None,
        maximum_state_bytes: None,
        maximum_scratch_bytes: None,
        maximum_automation_spans: None,
        maximum_reply_bytes: None,
    };
    let model = model_at(48_000);
    let mut browser = Rig::new(&model);
    let mut c_abi = CAbi::new(&model);
    // One frame, sent to both hosts, with a request ID past both legs' counters (the C ABI leg
    // pages its control dumps in more frames), which the C ABI leg's counter then takes: each host
    // sees ascending IDs, and both see the same frame.
    let next_frame = |browser: &mut Rig, c_abi: &mut CAbi| loop {
        let frame = browser.encode(CommandPayload::ResponseGet(&request), ExpectedRevision::Any);
        let id = u64::from_le_bytes(frame[24..32].try_into().expect("request ID"));
        if id > c_abi.request {
            c_abi.request = id;
            break frame;
        }
    };
    let frame = next_frame(&mut browser, &mut c_abi);
    // The unpatched request is answered alike.
    let (b, c) = (browser.send_frame(&frame), c_abi.send_raw(&frame));
    assert_eq!(b, c, "one answer to the unpatched request");
    assert_eq!(b.0, C_RESULT_OK);
    let before_b = dump(|payload, expected| browser.command(payload, expected));
    let before_c = dump(|payload, expected| c_abi.send(payload, expected));
    assert_eq!(before_b, before_c);
    let patched = |frame: Vec<u8>, from: f32, to: f32| {
        let from = from.to_le_bytes();
        let at: Vec<usize> = frame
            .windows(4)
            .enumerate()
            .filter(|(_, window)| *window == from)
            .map(|(at, _)| at)
            .collect();
        assert_eq!(at.len(), 1, "the value is in the frame exactly once");
        let mut bytes = frame;
        bytes[at[0]..at[0] + 4].copy_from_slice(&to.to_le_bytes());
        bytes
    };
    for (name, from, to, code) in [
        ("a NaN minimum", 20.0, f32::NAN, "response.grid"),
        (
            "an infinite maximum",
            20_000.0,
            f32::INFINITY,
            "response.grid",
        ),
        (
            "a NaN override value",
            3.25,
            f32::NAN,
            "response.override.value",
        ),
        ("a bypass of one half", 1.0, 0.5, "response.override.bypass"),
    ] {
        let bytes = patched(next_frame(&mut browser, &mut c_abi), from, to);
        let (b, c) = (browser.send_frame(&bytes), c_abi.send_raw(&bytes));
        assert_eq!(b, c, "{name}: one answer on both hosts");
        assert_eq!(b.0, C_RESULT_OK, "{name}: a refusal is a non-OK frame");
        let mut fields = [0_u16; 1024];
        let DecodedTypedResponseFrame::NonOk { header, payload } = ProtocolCodec::default()
            .decode_typed_response(&b.1, &mut DecodeScratch::new(&mut fields))
            .expect("a typed response")
        else {
            panic!("{name}: expected a refusal");
        };
        assert_eq!(header.status, protocol::StatusCode::InvalidField, "{name}");
        let codes: Vec<_> = payload
            .diagnostics
            .iter()
            .map(|d| d.code.as_str())
            .collect();
        assert_eq!(codes, [code], "{name}");
        assert_eq!(
            dump(|payload, expected| browser.command(payload, expected)),
            before_b,
            "{name}: the browser host is unchanged"
        );
        assert_eq!(
            dump(|payload, expected| c_abi.send(payload, expected)),
            before_c,
            "{name}: the C ABI is unchanged"
        );
    }
}

/// Block the bypass of `a_live_bypass_crossfades_alike_on_both_hosts` is submitted before.
const BYPASS_BLOCK: u64 = 6;

/// A live bypass (`edit`) through the C ABI and the browser host, at each launch rate: the blocks
/// are equal bit for bit (`run` compares them), and against two controls rendered by the browser
/// leg (never bypassed; authored bypassed by `author`) the render equals the first control until
/// the ramp reaches the output, differs from both for as many blocks as the session's `mute_ms`
/// ramp spans, and equals the second from then on.
fn a_live_bypass_crossfades_alike(edit: Edit, author: fn(&mut SessionModel)) {
    let step = [live(BYPASS_BLOCK, edit)];
    for rate in RATES {
        let (edited, _) = run(rate, &step);
        let model = model_at(rate);
        let mut plain = Rig::new(&model);
        let wet: Vec<Vec<u32>> = (0..BLOCKS).map(|block| plain.step(block)).collect();
        let mut dry_model = model.clone();
        author(&mut dry_model);
        let mut booted = Rig::new(&dry_model);
        let dry: Vec<Vec<u32>> = (0..BLOCKS).map(|block| booted.step(block)).collect();
        assert_ne!(wet[10], dry[10], "{rate} Hz: the bypass is audible");
        // The session's PDC delays the strip's output by the chain's latency, so the render of
        // the edit begins some blocks after the submit: it is found, not assumed.
        let start = (0..BLOCKS as usize)
            .find(|&block| edited[block] != wet[block])
            .expect("the bypass is heard");
        assert!(
            start >= BYPASS_BLOCK as usize,
            "{rate} Hz: the render changes at block {start}, before the submit at {BYPASS_BLOCK}"
        );
        // The default session `mute_ms` is 10 ms.
        let ramp_blocks = (u64::from(rate) / 100).div_ceil(u64::from(QUANTUM)) as usize;
        let between = (start..BLOCKS as usize)
            .take_while(|&block| edited[block] != dry[block])
            .count();
        assert!(
            (ramp_blocks..=ramp_blocks + 1).contains(&between),
            "{rate} Hz: {between} blocks sit between the controls, the 10 ms ramp spans {ramp_blocks} \
             or {}; a step spans at most one",
            ramp_blocks + 1
        );
        for block in start + between..BLOCKS as usize {
            assert_eq!(
                edited[block], dry[block],
                "{rate} Hz: block {block} follows the ramp"
            );
        }
    }
}

/// A live bypass of the compressor insert of `t1`.
///
/// Red if either host steps the bypass (a step spans at most one block), drops its ramp (the
/// same), runs it over a length other than the session's (the settle block moves), or leaves the
/// insert un-bypassed after the ramp.
#[test]
fn a_live_bypass_crossfades_alike_on_both_hosts() {
    a_live_bypass_crossfades_alike(
        Edit::Bypass {
            insert: COMPRESSOR,
            bypassed: true,
        },
        |model| {
            model
                .tracks
                .iter_mut()
                .find(|track| track.id.as_str() == COMPRESSOR.track.id)
                .expect("the compressor's track")
                .inserts
                .effects
                .iter_mut()
                .find(|effect| effect.id.as_str() == COMPRESSOR.id)
                .expect("the compressor")
                .bypass = true;
        },
    );
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
        if !matches!(full[omitted].outcome, Outcome::Accepted | Outcome::Rebuilt)
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
    c_abi_compile_document(&canonical_session_json(model).expect("canonical session"))
}

/// [`c_abi_compile`] for a session document that need not be one the model can write.
fn c_abi_compile_document(document: &str) -> (u32, Vec<u8>) {
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
    browser_boot_refusal_document(
        &canonical_session_json(model).expect("canonical session"),
        model.sample_rate_hz,
    )
}

/// [`browser_boot_refusal`] for a session document that need not be one the model can write.
fn browser_boot_refusal_document(document: &str, sample_rate_hz: u32) -> Option<Vec<u8>> {
    let options = host_web::WebBootOptions {
        require_sample_rate_hz: sample_rate_hz,
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

// ---------------------------------------------------------------------------------------------
// A rebuild adds a source the boot session did not declare (#1562b review B1)
// ---------------------------------------------------------------------------------------------

/// The longest source ID the grammar allows.
fn longest_id() -> String {
    format!("s{}", "x".repeat(session::STABLE_ID_MAXIMUM_BYTES - 1))
}

/// A session whose only source is mono (ID `m`), with one track and one route, no console slot,
/// no submix and no VCA, and the fixture's stereo source, track and route to add later.
fn mono_model(
    rate: u32,
) -> (
    SessionModel,
    session::Source,
    session::Track,
    session::Route,
) {
    let mut model = model_at(rate);
    let (source, mut track, route) = (
        model.sources[0].clone(),
        model.tracks[0].clone(),
        model.routes[0].clone(),
    );
    track.console.clear();
    track.inserts.effects.clear();
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    model.submixes.clear();
    model.vcas.clear();
    let (mut mono_source, mut mono_track, mut mono_route) =
        (source.clone(), track.clone(), route.clone());
    mono_source.id = id("m");
    mono_source.channels = 1;
    mono_track.id = id("m");
    mono_track.source_id = id("m");
    mono_track.left_source_channel = 0;
    mono_track.right_source_channel = 0;
    mono_route.id = id("m-main");
    let session::RouteSource::Track { track_id, .. } = &mut mono_route.source else {
        panic!("the fixture routes a track")
    };
    *track_id = id("m");
    model.sources = vec![mono_source];
    model.tracks = vec![mono_track];
    model.routes = vec![mono_route];
    (model, source, track, route)
}

/// Both hosts boot a session whose only source is mono; a rebuild then adds a stereo source whose ID has the
/// longest length the grammar allows; the browser lists it in its session map and takes its PCM,
/// and both hosts render the same bits for it.
///
/// Test value: red if a rebuild that adds a source (the longest ID, stereo, after a mono-only boot)
/// is rendered or fed differently by the browser than by the C ABI: the source rows, the swap, the
/// fresh-generation seek and the chunk path agree bit for bit. The staging the exports use for the
/// same session is gated in `plan_exchange_threads.rs`, since this row drives the host's safe API.
#[test]
fn a_rebuild_adds_the_longest_stereo_source_to_a_session_that_booted_with_a_mono_source_only() {
    for rate in RATES {
        let (model, mut source, mut track, mut route) = mono_model(rate);
        let name = longest_id();
        source.id = id(&name);
        source.channels = 2;
        source.frames = u64::from(QUANTUM) * (BLOCKS + 8);
        track.id = id("t1");
        track.source_id = source.id.clone();
        route.id = id("t1-main");
        let session::RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("the fixture routes a track")
        };
        *track_id = track.id.clone();
        let edits = [
            SessionEdit::UpsertSource { source },
            SessionEdit::UpsertTrack { track },
            SessionEdit::UpsertRoute { route },
        ];

        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        assert_eq!(
            browser.host.session_source_count(),
            1,
            "{rate} Hz: boots mono"
        );
        for block in 0..2 {
            assert_eq!(browser.host.render_next(), host_web::RESULT_OK);
            let _ = c_abi.render(block);
        }
        let (rebuilt, b_reply) = browser.transaction_reply(&edits);
        let c_reply = c_abi.send(
            CommandPayload::SessionTransactionApply(&edits),
            ExpectedRevision::Exact(SessionRevision(c_abi.revision)),
        );
        assert_eq!(b_reply, c_reply, "{rate} Hz: the same reply bytes");
        assert!(
            matches!(
                rebuilt,
                Verdict::Applied {
                    path: EditPath::Rebuild,
                    ..
                }
            ),
            "{rate} Hz: adding a source is a rebuild, got {rebuilt:?}"
        );
        // The swap block: the successor adopts at the next block boundary.
        assert_eq!(browser.host.render_next(), host_web::RESULT_OK);
        let _ = c_abi.render(2);

        assert_eq!(
            browser.host.session_source_count(),
            2,
            "{rate} Hz: the map lists it"
        );
        // Canonical order is by ID: `m` then the long one.
        assert_eq!(browser.host.session_source_id(1), Some(name.as_str()));
        assert_eq!(
            browser
                .host
                .session_source_shape(1)
                .map(|shape| shape.channel_count),
            Some(2)
        );

        // The source enters at the render clock the successor adopted at, so it starts at its own
        // frame 0 from block 3 on: a seek to a new generation, then the chunks of that generation.
        assert_eq!(
            browser.host.seek_source(name.as_bytes(), 2, 0),
            host_web::RESULT_OK
        );
        // SAFETY: The session is live and used by this thread alone; the name outlives the call.
        let code = unsafe {
            capi::miso_engine_v1_source_seek(c_abi.session, name.as_ptr(), name.len() as u64, 2, 0)
        };
        assert_eq!(code, C_RESULT_OK, "{rate} Hz: C ABI seek");
        let mut heard = false;
        for block in 3..11 {
            let [left, right] = planes(0, block);
            assert_eq!(
                browser.host.submit_source(
                    name.as_bytes(),
                    2,
                    (block - 3) * u64::from(QUANTUM),
                    rate,
                    &[&left, &right],
                    QUANTUM,
                    false,
                ),
                host_web::RESULT_OK,
                "{rate} Hz: the browser takes the new source's PCM"
            );
            c_abi.submit_from(&name, 0, block, block - 3, 2);
            assert_eq!(browser.host.render_next(), host_web::RESULT_OK);
            let b = bits(browser.host.output_pcm().expect("browser output"));
            let c = c_abi.render(block);
            assert_eq!(b, c, "{rate} Hz: block {block}: the two hosts render alike");
            heard |= b.iter().any(|word| f32::from_bits(*word) != 0.0);
        }
        assert!(heard, "{rate} Hz: the new source is heard");
    }
}

// ---------------------------------------------------------------------------------------------
// A source is mono or stereo (owner answer of 2026-10-15)
// ---------------------------------------------------------------------------------------------

/// Both hosts refuse a session whose source declares three channels at load, and a structural edit
/// that adds one before it commits, with the same typed code and the same reply bytes; a mono and a
/// stereo source load as before.
///
/// Test value: red if one host's load or structural-edit path lets a wider-than-stereo source
/// through (the browser sizes its source staging for two channels, so a three-channel source it
/// accepted could never be fed), if the two hosts name different codes, or if a refused edit moves
/// the committed revision.
#[test]
fn both_hosts_refuse_a_source_of_more_than_two_channels_alike() {
    for rate in RATES {
        // At load. The model cannot write a three-channel source, so the document is edited.
        let (model, source, ..) = mono_model(rate);
        let document = canonical_session_json(&model).expect("a mono session");
        let stereo = {
            let mut model = model.clone();
            model.sources[0].channels = 2;
            model.tracks[0].right_source_channel = 1;
            canonical_session_json(&model).expect("a stereo session")
        };
        assert_eq!(
            c_abi_compile_document(&document).0,
            C_RESULT_OK,
            "{rate} mono"
        );
        assert_eq!(browser_boot_refusal_document(&document, rate), None);
        assert_eq!(
            c_abi_compile_document(&stereo).0,
            C_RESULT_OK,
            "{rate} stereo"
        );
        assert_eq!(browser_boot_refusal_document(&stereo, rate), None);
        let wide = stereo.replacen(
            "\"channels\": 2,\n      \"bit_depth\"",
            "\"channels\": 3,\n      \"bit_depth\"",
            1,
        );
        assert_ne!(wide, stereo, "{rate}: the source declaration is found");
        let (code, c_diagnostics) = c_abi_compile_document(&wide);
        assert_ne!(
            code, C_RESULT_OK,
            "{rate}: the C ABI refuses three channels"
        );
        let browser_diagnostics =
            browser_boot_refusal_document(&wide, rate).expect("the browser refuses");
        assert_eq!(c_diagnostics, browser_diagnostics, "{rate}: one diagnostic");
        assert!(
            String::from_utf8_lossy(&c_diagnostics).contains("source.channels.unsupported"),
            "{rate}: the typed code, got {:?}",
            String::from_utf8_lossy(&c_diagnostics)
        );

        // In a structural edit: refused before commit, the revision does not move.
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        let mut wide_source = source;
        wide_source.id = id("wide");
        wide_source.channels = 3;
        let edits = [SessionEdit::UpsertSource {
            source: wide_source,
        }];
        let (verdict, b_reply) = browser.transaction_reply(&edits);
        let c_reply = c_abi.send(
            CommandPayload::SessionTransactionApply(&edits),
            ExpectedRevision::Exact(SessionRevision(c_abi.revision)),
        );
        assert_eq!(b_reply, c_reply, "{rate}: the same reply bytes");
        assert!(
            matches!(&verdict, Verdict::Refused { codes, revision }
                if *revision == model.revision
                    && codes.as_slice() == ["source.channels.unsupported"]),
            "{rate}: refused at the committed revision, got {verdict:?}"
        );
        assert_eq!(
            browser.host.session_source_count(),
            1,
            "{rate}: nothing committed"
        );
    }
}

/// #1648a (D1): adding stored fader automation (builtin row 5, a row that renders) is a rebuild
/// on both hosts, with the same reply bytes, and both hosts render the same bits across the swap;
/// an entry on a row that does not render yet (the mute, row 6) commits `model_only` on both.
/// Both hosts call the one shared classifier, so neither has its own rule for automation edits.
///
/// A changed ride and its removal then rebuild alike too, and both hosts render the same bits
/// through the carried cell's adoption jump and the release (#1648b).
///
/// Test value: red if either host acks a fader automation edit as `model_only` (an edit that is
/// never heard), if the two hosts answer it differently, or if the browser's rebuild renders other
/// bits than the C ABI's.
#[test]
fn a_fader_automation_edit_rebuilds_alike_on_both_hosts() {
    let entry = |name: &str, control: u32, segment: session::AutomationSegment| {
        SessionEdit::UpsertAutomation {
            automation: session::Automation {
                id: id(name),
                target: address(
                    Owner::Strip(id(T1.id)),
                    Section::Builtins,
                    control,
                    ParameterChannel::Both,
                ),
                segments: vec![segment],
                offset: session::AutomationOffset::default(),
            },
        }
    };
    let fader = entry(
        "t1-fader-ride",
        5,
        session::AutomationSegment {
            shape: session::AutomationShape::Linear,
            start_sample: 0,
            end_sample: 960,
            start_value: 0.0,
            end_value: -6.0,
            unit: session::ParameterUnit::Db,
        },
    );
    let changed = entry(
        "t1-fader-ride",
        5,
        session::AutomationSegment {
            shape: session::AutomationShape::Linear,
            start_sample: 0,
            end_sample: 9_600,
            start_value: 0.0,
            end_value: -18.0,
            unit: session::ParameterUnit::Db,
        },
    );
    let mute = entry(
        "t1-mute-ride",
        6,
        session::AutomationSegment {
            shape: session::AutomationShape::Step,
            start_sample: 0,
            end_sample: 960,
            start_value: 0.0,
            end_value: 0.0,
            unit: session::ParameterUnit::Linear,
        },
    );
    for rate in RATES {
        let model = model_at(rate);
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        same_blocks(&mut browser, &mut c_abi, 0..3, "before the edit");
        let (b, b_reply) = browser.transaction_reply(std::slice::from_ref(&fader));
        let (c, c_reply) = c_abi.transaction(std::slice::from_ref(&fader));
        assert_eq!(b, c, "{rate} Hz: one verdict for the fader entry");
        assert_eq!(b_reply, c_reply, "{rate} Hz: one reply");
        assert_eq!(
            b,
            Verdict::Applied {
                path: EditPath::Rebuild,
                revision: model.revision + 1
            },
            "{rate} Hz: a fader entry rebuilds"
        );
        same_blocks(&mut browser, &mut c_abi, 3..8, "across the swap");
        let (b, b_reply) = browser.transaction_reply(std::slice::from_ref(&mute));
        let (c, c_reply) = c_abi.transaction(std::slice::from_ref(&mute));
        assert_eq!(b, c, "{rate} Hz: one verdict for the mute entry");
        assert_eq!(b_reply, c_reply, "{rate} Hz: one reply");
        assert_eq!(
            b,
            Verdict::Applied {
                path: EditPath::ModelOnly,
                revision: model.revision + 2
            },
            "{rate} Hz: a mute entry is model-only"
        );
        same_blocks(&mut browser, &mut c_abi, 8..10, "after the mute entry");
        // #1648b (gate 2's browser leg): a changed ride and a removed ride rebuild alike, and the
        // carried cell's adoption jump and the release render the same bits on both hosts.
        for (step, edit, blocks) in [
            (3, changed.clone(), 10..18),
            (
                4,
                SessionEdit::RemoveAutomation {
                    automation_id: id("t1-fader-ride"),
                },
                18..26,
            ),
        ] {
            let (b, b_reply) = browser.transaction_reply(std::slice::from_ref(&edit));
            let (c, c_reply) = c_abi.transaction(std::slice::from_ref(&edit));
            assert_eq!(b, c, "{rate} Hz: one verdict for edit {step}");
            assert_eq!(b_reply, c_reply, "{rate} Hz: one reply for edit {step}");
            assert_eq!(
                b,
                Verdict::Applied {
                    path: EditPath::Rebuild,
                    revision: model.revision + step
                },
                "{rate} Hz: edit {step} rebuilds"
            );
            same_blocks(&mut browser, &mut c_abi, blocks, "across the carried swap");
        }
    }
}

/// #1631. A `matrix_mode` switch under a stored pan entry is refused alike by both hosts: the same
/// refusal and the same diagnostic (`control.inactive` at the entry's path, the row rule's
/// own, through the rebuild route), and nothing commits -- the same expected revision still
/// commits the switch together with the entry's retarget to the matrix row, without a rebuild, on both.
///
/// Red mutation: run the classifier's row rule only when the automation table changes -> the switch
/// commits live on both hosts and the first assertion fails.
#[test]
fn a_mode_switch_under_a_stored_pan_entry_is_refused_alike_on_both_hosts() {
    for rate in RATES {
        let mut model = model_at(rate);
        model.automation.push(session::Automation {
            id: id("t2-pan-ride"),
            target: address(
                Owner::Strip(id(T2.id)),
                Section::Builtins,
                PAN_ROW,
                ParameterChannel::Left,
            ),
            segments: vec![session::AutomationSegment {
                shape: session::AutomationShape::Linear,
                start_sample: 0,
                end_sample: 960,
                start_value: 0.0,
                end_value: 0.5,
                unit: session::ParameterUnit::Linear,
            }],
            offset: session::AutomationOffset::default(),
        });
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        let switch = CAbi::edits(
            Edit::MatrixMode {
                strip: T2,
                matrix: true,
            },
            None,
        );

        let frame = browser.encode(
            CommandPayload::SessionTransactionApply(&switch),
            ExpectedRevision::Exact(SessionRevision(browser.revision)),
        );
        let (b_code, b_diagnostic) = browser.send_frame(&frame);
        let (c_code, c_diagnostic) = c_abi.send_status(
            CommandPayload::SessionTransactionApply(&switch),
            ExpectedRevision::Exact(SessionRevision(c_abi.revision)),
        );
        assert_ne!(
            b_code, C_RESULT_OK,
            "{rate}: the browser refuses the switch"
        );
        // Each host has its own result numbering; the diagnostic is the shared contract.
        assert_ne!(c_code, C_RESULT_OK, "{rate}: the C ABI refuses the switch");
        assert_eq!(b_diagnostic, c_diagnostic, "{rate}: one diagnostic");
        let diagnostic = String::from_utf8_lossy(&c_diagnostic);
        assert!(
            diagnostic.contains("control.inactive")
                && diagnostic.contains("$.automation[id=t2-pan-ride].target"),
            "{rate}: {diagnostic}"
        );

        // Nothing committed: the revision the refused transaction named still commits, with the
        // entry moved to the matrix row in the same transaction.
        let mut retargeted = switch.clone();
        retargeted.push(SessionEdit::SetAutomationTarget {
            automation_id: id("t2-pan-ride"),
            target: address(
                Owner::Strip(id(T2.id)),
                Section::Builtins,
                MATRIX_LL_ROW,
                ParameterChannel::Both,
            ),
        });
        let (b_verdict, b_reply) = browser.transaction_reply(&retargeted);
        let c_reply = c_abi.send(
            CommandPayload::SessionTransactionApply(&retargeted),
            ExpectedRevision::Exact(SessionRevision(c_abi.revision)),
        );
        assert_eq!(b_reply, c_reply, "{rate}: the same reply bytes");
        assert!(
            // The switch keeps the effective 2x2 the plan holds, so it writes no record.
            matches!(&b_verdict, Verdict::Applied { path: EditPath::ModelOnly, revision }
                if *revision == model.revision + 1),
            "{rate}: committed without a rebuild at the next revision, got {b_verdict:?}"
        );
    }
}

// ---------------------------------------------------------------------------------------------
// An empty session boots and grows by a structural transaction (#1682)
// ---------------------------------------------------------------------------------------------

/// Both hosts boot a session with no source and no track (one console slot stays declared), render
/// silence, and report one empty mixer; one structural transaction then adds a stereo stem, its
/// track and its route, and both hosts render the same bits for the fed stem. The C ABI leg alone
/// is `crates/capi/tests/empty_boot.rs`.
///
/// Test value: red if the browser has a boot-time size that fails at zero (a strip order, a meter
/// frame, a source map, an observation table or a staging buffer sized for the first strip), if its
/// silence or its session map, control list or snapshot differ from the C ABI's, or if the first
/// strip cannot arrive through the successor paths and be heard bit for bit like the C ABI's.
#[test]
fn both_hosts_boot_an_empty_session_and_add_the_first_stem_alike() {
    for rate in RATES {
        let mut model = model_at(rate);
        let (mut source, mut track, mut route) = (
            model.sources[0].clone(),
            model.tracks[0].clone(),
            model.routes[0].clone(),
        );
        model.sources.clear();
        model.tracks.clear();
        model.routes.clear();
        model.vcas.clear();
        assert_eq!(model.console.pre_insert.len(), 1, "a declared console slot");
        source.id = id("stem");
        source.frames = u64::from(QUANTUM) * (BLOCKS + 8);
        track.id = id("t1");
        track.source_id = source.id.clone();
        route.id = id("t1-main");
        let session::RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("the fixture routes a track")
        };
        *track_id = track.id.clone();
        let edits = [
            SessionEdit::UpsertSource { source },
            SessionEdit::UpsertTrack { track },
            SessionEdit::UpsertRoute { route },
        ];

        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        assert_eq!(browser.host.session_source_count(), 0, "{rate} Hz: empty");
        for block in 0..3 {
            assert_eq!(browser.host.render_next(), host_web::RESULT_OK);
            let b = bits(browser.host.output_pcm().expect("browser output"));
            let c = c_abi.render(block);
            assert_eq!(b, c, "{rate} Hz: block {block}: the empty mixers agree");
            assert!(b.iter().all(|word| *word == 0), "{rate} Hz: silence");
        }
        let (b_rows, b_snapshot) = dump(|payload, expected| browser.command(payload, expected));
        let (c_rows, c_snapshot) = dump(|payload, expected| c_abi.send(payload, expected));
        assert_eq!(b_rows, c_rows, "{rate} Hz: the empty control list agrees");
        assert_eq!(
            b_snapshot, c_snapshot,
            "{rate} Hz: the empty snapshot agrees"
        );

        let (rebuilt, _) = browser.transaction_reply(&edits);
        let c_reply = c_abi.send(
            CommandPayload::SessionTransactionApply(&edits),
            ExpectedRevision::Exact(SessionRevision(c_abi.revision)),
        );
        // The two legs have sent different numbers of frames, so their request IDs differ: the
        // verdicts are compared, not the bytes.
        assert_eq!(
            rebuilt,
            decode_verdict(&c_reply),
            "{rate} Hz: the same verdict"
        );
        assert!(
            matches!(
                rebuilt,
                Verdict::Applied {
                    path: EditPath::Rebuild,
                    ..
                }
            ),
            "{rate} Hz: the first stem is a rebuild, got {rebuilt:?}"
        );
        assert_eq!(browser.host.render_next(), host_web::RESULT_OK);
        let _ = c_abi.render(3);
        assert_eq!(browser.host.session_source_count(), 1, "{rate} Hz: the map");
        assert_eq!(browser.host.session_source_id(0), Some("stem"));

        assert_eq!(browser.host.seek_source(b"stem", 2, 0), host_web::RESULT_OK);
        // SAFETY: The session is live and used by this thread alone; the name outlives the call.
        let code =
            unsafe { capi::miso_engine_v1_source_seek(c_abi.session, b"stem".as_ptr(), 4, 2, 0) };
        assert_eq!(code, C_RESULT_OK, "{rate} Hz: C ABI seek");
        let mut heard = false;
        for block in 4..12 {
            let [left, right] = planes(0, block);
            assert_eq!(
                browser.host.submit_source(
                    b"stem",
                    2,
                    (block - 4) * u64::from(QUANTUM),
                    rate,
                    &[&left, &right],
                    QUANTUM,
                    false,
                ),
                host_web::RESULT_OK,
                "{rate} Hz: the browser takes the first stem's PCM"
            );
            c_abi.submit_from("stem", 0, block, block - 4, 2);
            assert_eq!(browser.host.render_next(), host_web::RESULT_OK);
            let b = bits(browser.host.output_pcm().expect("browser output"));
            let c = c_abi.render(block);
            assert_eq!(b, c, "{rate} Hz: block {block}: the two hosts render alike");
            heard |= b.iter().any(|word| f32::from_bits(*word) != 0.0);
        }
        assert!(heard, "{rate} Hz: the first stem is heard");
    }
}

/// #1683: the master designation is the core's `master` monitor row on both hosts. Both boot an
/// empty session, grow a track and a submix by one rebuild, then designate, move and clear the
/// master with `SET_CONTROL`, and refuse a strip neither has: each transaction's verdict, the
/// control list and state pages (the row's value) and the snapshot (which never holds it) agree.
///
/// Test value: red if one host lacks the row, refuses a designation the other takes, spends a
/// revision on it, lists another master, holds two masters at once, saves the designation into
/// the snapshot, or admits a strip the session does not have.
#[test]
fn both_hosts_designate_the_master_strip_after_an_empty_boot_alike() {
    let rate = RATES[0];
    let mut model = model_at(rate);
    let (mut source, mut track, mut route) = (
        model.sources[0].clone(),
        model.tracks[0].clone(),
        model.routes[0].clone(),
    );
    model.sources.clear();
    model.tracks.clear();
    model.routes.clear();
    model.vcas.clear();
    source.id = id("stem");
    track.id = id("t1");
    track.source_id = source.id.clone();
    route.id = id("t1-main");
    let session::RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("the fixture routes a track")
    };
    *track_id = track.id.clone();
    let bus = session::Submix::unity(id("bus"), &model.console);
    let master = |strip: &str, on: bool| SessionEdit::SetControl {
        address: address(Owner::Strip(id(strip)), Section::Own, 4, Lane::Both),
        value: if on { 1.0 } else { 0.0 },
        ramp_samples: None,
        expected_effect: None,
    };
    let transactions = [
        vec![
            SessionEdit::UpsertSource { source },
            SessionEdit::UpsertTrack { track },
            SessionEdit::UpsertRoute { route },
            SessionEdit::UpsertSubmix { submix: bus },
        ],
        vec![master("t1", true)],
        vec![master("bus", true)],
        vec![master("nobody", true)],
        vec![master("t1", false)],
        vec![master("bus", false)],
    ];

    let mut browser = Rig::new(&model);
    let mut c_abi = CAbi::new(&model);
    for (step, edits) in transactions.iter().enumerate() {
        let before = c_abi.revision;
        let (b, _) = browser.transaction_reply(edits);
        let (c, _) = c_abi.transaction(edits);
        assert_eq!(b, c, "step {step}: the same verdict");
        match step {
            0 => assert!(
                matches!(
                    b,
                    Verdict::Applied {
                        path: EditPath::Rebuild,
                        ..
                    }
                ),
                "{b:?}"
            ),
            3 => assert!(matches!(b, Verdict::Refused { .. }), "{b:?}"),
            _ => assert_eq!(
                b,
                Verdict::Applied {
                    path: EditPath::Monitor,
                    revision: before,
                },
                "step {step}: a monitor write spends no revision"
            ),
        }
        assert_eq!(browser.host.render_next(), host_web::RESULT_OK);
        let _ = c_abi.render(step as u64);
        let (b_rows, b_snapshot) = dump(|payload, expected| browser.command(payload, expected));
        let (c_rows, c_snapshot) = dump(|payload, expected| c_abi.send(payload, expected));
        assert_eq!(b_rows, c_rows, "step {step}: the control pages agree");
        assert_eq!(b_snapshot, c_snapshot, "step {step}: the snapshots agree");
        // The listed masters: the owner of each `master` row whose value is on, by the rows'
        // typed fields, on each host.
        let expected: &[&str] = match step {
            1 => &["t1"],
            2..=4 => &["bus"],
            _ => &[],
        };
        assert_eq!(
            listed_masters(|payload, expected| browser.command(payload, expected)),
            expected,
            "step {step}: the browser's listed master"
        );
        assert_eq!(
            listed_masters(|payload, expected| c_abi.send(payload, expected)),
            expected,
            "step {step}: the C ABI's listed master"
        );
    }
}

/// #1310 on a paused host (batch 15a review fold B1): eight structural edits with no render
/// between them each commit on both hosts, each superseding the candidate before it, and from the
/// block that adopts the newest plan the two hosts render the same bits.
///
/// Test value: red if one host refuses a paused structural edit the other commits (the browser
/// kept a superseded candidate's tables until a later adoption and refused the fifth edit with
/// `BACKPRESSURE`), or if the two hosts adopt different plans.
#[test]
fn paused_structural_edits_commit_alike_on_both_hosts() {
    const EDITS: u64 = 8;
    const PAUSE_AT: u64 = 2;
    for rate in [48_000, 96_000] {
        let model = model_at(rate);
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        for block in 0..PAUSE_AT {
            let b = browser.step(block);
            assert_eq!(b, c_abi.step(block), "{rate} Hz: block {block}");
        }
        let routes: Vec<session::Route> = model
            .routes
            .iter()
            .filter(|route| {
                matches!(
                    &route.source,
                    session::RouteSource::Track { track_id, .. } if track_id.as_str() == T1.id
                )
            })
            .cloned()
            .collect();
        assert!(!routes.is_empty(), "t1 routes somewhere");
        for edit in 0..EDITS {
            // A copy of `t1` with no insert, on `t1`'s source, routed as `t1` is.
            let mut track = model.tracks[T1.index as usize].clone();
            track.id = id(&format!("p{edit}"));
            track.inserts.effects.clear();
            let mut edits = vec![SessionEdit::UpsertTrack {
                track: track.clone(),
            }];
            for (index, route) in routes.iter().enumerate() {
                let mut route = route.clone();
                route.id = id(&format!("p{edit}-r{index}"));
                let session::RouteSource::Track { track_id, .. } = &mut route.source else {
                    unreachable!("filtered on a track source")
                };
                *track_id = track.id.clone();
                edits.push(SessionEdit::UpsertRoute { route });
            }
            let (b_verdict, _) = browser.transaction_reply(&edits);
            let (c_verdict, _) = c_abi.transaction(&edits);
            c_abi.drain_reliable();
            assert_eq!(
                b_verdict,
                Verdict::Applied {
                    path: EditPath::Rebuild,
                    revision: model.revision + edit + 1
                },
                "{rate} Hz: paused edit {edit} commits on the browser"
            );
            assert_eq!(b_verdict, c_verdict, "{rate} Hz: paused edit {edit}");
        }
        for block in PAUSE_AT..PAUSE_AT + 8 {
            let b = browser.step(block);
            assert_eq!(b, c_abi.step(block), "{rate} Hz: block {block}");
        }
        assert_eq!(
            browser.host.live_control_tracks().len(),
            model.tracks.len() + EDITS as usize,
            "{rate} Hz: the browser adopted the plan of every edit"
        );
    }
}

/// #1310, root rulings of 2026-10-16 (no ack before a drop; M-A of fold 3), on both hosts. T1 (A)
/// redeclares `src-t1`, so A restarts it at frame 0 in a ring A creates, and the host's next
/// chunk for it is acked into A's ring. T2 (B) does not keep `src-t1` as A declares it: (a) it
/// puts `src-t1` back as the running plan declares it, (b) it redeclares it a third way, or (c)
/// it removes `src-t1` with its track `t1` and `t1`'s routes. B does not take A's ring by
/// donation, so A's acked chunk would drop with A, and both hosts refuse T2 with
/// `BACKPRESSURE`, the same code and the same revision. The next block adopts A on both, and the
/// retried T2 commits alike; the two hosts render the same bits in every block.
///
/// This session's plan has a compensated latency of more than one block, and it grows with the
/// rate (a plain run first plays at frame 459 at 48 kHz, 939 at 96 kHz): the first block a plain
/// run plays is `lead`. So T1 commits only once the output plays, the adoption block must
/// play, and the acked chunk reaches the output `lead` blocks after the adoption block. The retry
/// waits for that block. A control run, whose host never feeds A's `src-t1` ring and so never
/// attempts the early T2, renders the same blocks until then and a different block there, on both
/// hosts: the acked chunk is heard, at A's restart position.
///
/// Test value: red if either host lets such a supersession commit (the guard missing in the
/// core, or the fold-2 guard that refused only case (a)), if the two hosts refuse it differently
/// or render differently, or if A's acked chunk is not heard (dropped, or not played at A's
/// restart position).
#[test]
fn a_supersession_that_would_drop_acked_pcm_is_backpressure_alike_on_both_hosts() {
    let plays = |pcm: &[u32]| pcm.iter().any(|word| f32::from_bits(*word) != 0.0);
    for rate in [48_000, 96_000] {
        let model = model_at(rate);
        let name = format!("src-{}", T1.id);
        let original = model
            .sources
            .iter()
            .find(|source| source.id.as_str() == name)
            .expect("t1's source")
            .clone();
        let redeclare = |way: u8| {
            let mut redeclared = original.clone();
            let last = redeclared.content.pop().expect("a content identity");
            // `way` distinct digits, none equal to the original's last one.
            let digit = (b'0'..=b'9')
                .filter(|&digit| char::from(digit) != last)
                .nth(usize::from(way))
                .expect("a digit");
            redeclared.content.push(char::from(digit));
            redeclared
        };
        let first = [SessionEdit::UpsertSource {
            source: redeclare(0),
        }];
        let mut removal: Vec<SessionEdit> = model
            .routes
            .iter()
            .filter(|route| {
                matches!(
                    &route.source,
                    session::RouteSource::Track { track_id, .. } if track_id.as_str() == T1.id
                )
            })
            .map(|route| SessionEdit::RemoveRoute {
                route_id: route.id.clone(),
            })
            .collect();
        removal.push(SessionEdit::RemoveTrack {
            track_id: id(T1.id),
        });
        removal.push(SessionEdit::RemoveSource {
            source_id: original.id.clone(),
        });
        for (case, second, t1_in_b) in [
            (
                "(a) reverted",
                vec![SessionEdit::UpsertSource {
                    source: original.clone(),
                }],
                true,
            ),
            (
                "(b) redeclared a third way",
                vec![SessionEdit::UpsertSource {
                    source: redeclare(1),
                }],
                true,
            ),
            ("(c) removed", removal.clone(), false),
        ] {
            // The plan's latency in whole blocks: the first block a plain run plays.
            let lead = {
                let mut browser = Rig::new(&model);
                (0..BLOCKS)
                    .find(|&block| plays(&browser.step(block)))
                    .expect("the session plays")
            };
            let adopt = lead + 1;
            let heard_at = adopt + lead;
            let retry = heard_at + 1;

            // Each host is fed `src-t1` (if `t1`) from `t1_start` (its own frame, in blocks) and the
            // other two sources at the render position, then renders `block`.
            let feed_and_render = |browser: &mut Rig,
                                   c_abi: &mut CAbi,
                                   block: u64,
                                   t1: bool,
                                   t1_start: u64|
             -> Vec<u32> {
                for (feed, strip) in [T1, T2, T3].into_iter().enumerate() {
                    if strip.id == T1.id && !t1 {
                        continue;
                    }
                    let source = format!("src-{}", strip.id);
                    let start = if strip.id == T1.id { t1_start } else { block };
                    let [left, right] = planes(feed as u64, block);
                    assert_eq!(
                        browser.host.submit_source(
                            source.as_bytes(),
                            1,
                            start * u64::from(QUANTUM),
                            rate,
                            &[&left, &right],
                            QUANTUM,
                            false,
                        ),
                        host_web::RESULT_OK,
                        "{rate} Hz {case}: browser {source}"
                    );
                    c_abi.submit_from(&source, feed as u64, block, start, 1);
                }
                assert_eq!(browser.host.render_next(), host_web::RESULT_OK);
                let b = bits(browser.host.output_pcm().expect("browser output"));
                assert_eq!(b, c_abi.render(block), "{rate} Hz {case}: block {block}");
                b
            };

            // `feed_a`: the host feeds A's `src-t1` ring the acked chunk, and T2 is attempted before
            // the adoption. Otherwise the host never feeds A's ring before the retry.
            let run = |feed_a: bool| -> Vec<Vec<u32>> {
                let mut browser = Rig::new(&model);
                let mut c_abi = CAbi::new(&model);
                let mut blocks = Vec::new();
                for block in 0..adopt {
                    let b = browser.step(block);
                    assert_eq!(b, c_abi.step(block), "{rate} Hz {case}: block {block}");
                    blocks.push(b);
                }

                let (b_first, _) = browser.transaction_reply(&first);
                let (c_first, _) = c_abi.transaction(&first);
                assert!(
                    matches!(
                        b_first,
                        Verdict::Applied {
                            path: EditPath::Rebuild,
                            ..
                        }
                    ),
                    "{rate} Hz {case}: the redeclaration is a rebuild: {b_first:?}"
                );
                assert_eq!(b_first, c_first, "{rate} Hz {case}: T1");
                let revision = c_abi.revision;

                if feed_a {
                    // A's `src-t1` chunk at frame 0, acked before any block adopts A: the source's
                    // PCM of the adoption block, as the host would send it.
                    let [left, right] = planes(0, adopt);
                    assert_eq!(
                        browser.host.submit_source(
                            name.as_bytes(),
                            1,
                            0,
                            rate,
                            &[&left, &right],
                            QUANTUM,
                            false,
                        ),
                        host_web::RESULT_OK
                    );
                    c_abi.submit_from(&name, 0, adopt, 0, 1);

                    // T2 before any block adopts A.
                    let b_frame = browser.encode(
                        CommandPayload::SessionTransactionApply(&second),
                        ExpectedRevision::Exact(SessionRevision(browser.revision)),
                    );
                    let (b_code, b_reply) = browser.send_frame(&b_frame);
                    let (c_code, c_reply) = c_abi.send_status(
                        CommandPayload::SessionTransactionApply(&second),
                        ExpectedRevision::Exact(SessionRevision(c_abi.revision)),
                    );
                    assert_eq!(
                        (b_code, c_code),
                        (host_web::RESULT_BACKPRESSURE, capi::RESULT_BACKPRESSURE),
                        "{rate} Hz {case}: the supersession would drop A's acked chunk: {} / {}",
                        String::from_utf8_lossy(&b_reply),
                        String::from_utf8_lossy(&c_reply)
                    );
                    assert_eq!(c_abi.revision, revision, "{rate} Hz {case}: no commit");
                    assert_eq!(browser.revision, revision, "{rate} Hz {case}: no commit");
                    assert_eq!(
                        drain_events(&mut browser.host),
                        0,
                        "{rate} Hz {case}: the refusal queues no event"
                    );
                }

                // The adoption block, and the blocks until the acked chunk reaches the output. Only
                // the other two sources are fed: the one difference between the two runs is the
                // acked chunk.
                for block in adopt..retry {
                    blocks.push(feed_and_render(&mut browser, &mut c_abi, block, false, 0));
                }

                // The retry commits on A, alike; B restarts `src-t1` at frame 0 again.
                let (b_second, _) = browser.transaction_reply(&second);
                let (c_second, _) = c_abi.transaction(&second);
                assert!(
                    matches!(b_second, Verdict::Applied { .. }),
                    "{rate} Hz {case}: the retry commits: {b_second:?}"
                );
                assert_eq!(b_second, c_second, "{rate} Hz {case}: T2 retried");
                for block in retry..retry + 5 {
                    blocks.push(feed_and_render(
                        &mut browser,
                        &mut c_abi,
                        block,
                        t1_in_b,
                        block - retry,
                    ));
                }
                blocks
            };

            let fed = run(true);
            let unfed = run(false);
            let adopt = adopt as usize;
            let heard_at = heard_at as usize;
            assert!(
                plays(&fed[adopt]),
                "{rate} Hz {case}: the adoption block (block {adopt}) plays"
            );
            assert!(
                plays(&fed[heard_at]),
                "{rate} Hz {case}: the block that carries the acked chunk (block {heard_at}) plays"
            );
            assert_eq!(
                fed[..heard_at],
                unfed[..heard_at],
                "{rate} Hz {case}: the refusal changes nothing, and the acked chunk is not heard early"
            );
            assert_ne!(
                fed[heard_at], unfed[heard_at],
                "{rate} Hz {case}: the acked chunk is heard (block {heard_at}, latency {lead} blocks)"
            );
            assert!(
                fed[retry as usize..].iter().all(|block| plays(block)),
                "{rate} Hz: the blocks after the retry play"
            );
        }
    }
}

/// The C ABI plan's `latency_samples`, through the exported resource query.
fn c_latency(c_abi: &CAbi) -> u64 {
    // SAFETY: Every field of the report is an integer, for which zero is a valid value.
    let mut report: capi::PlanResourceReport = unsafe { core::mem::zeroed() };
    report.struct_size = capi::PLAN_RESOURCE_REPORT_SIZE;
    // SAFETY: The plan is live; `report` is one complete local report.
    let code = unsafe { capi::miso_engine_v1_plan_resources(c_abi.plan, &mut report) };
    assert_eq!(code, C_RESULT_OK);
    report.latency_samples
}

/// #1323 on both hosts: removing the true-peak limiter insert keeps every node's latency (#1285),
/// and a declared discontinuity, `miso_engine_v1_declare_discontinuity` on the C ABI and the
/// browser host's `declare_discontinuity`, publishes the committed session with no floors on both.
/// From the block after it the two hosts render the same bits, and the C ABI reports a lower
/// latency than before the declaration.
///
/// Test value: red if one host's entry does not reach the control plane's declaration, or
/// reaches another rule (that host keeps the floors, and its remaining tracks play later than the
/// other host's), or if the declaration is a no-op on both (the C ABI latency stays floored).
#[test]
fn a_declared_discontinuity_resets_the_floors_alike_on_both_hosts() {
    for rate in [48_000, 96_000] {
        let model = model_at(rate);
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        same_blocks(&mut browser, &mut c_abi, 0..4, &format!("{rate} Hz before"));
        let edits = [SessionEdit::RemoveTrackEffect {
            track_id: id(LIMITER.track.id),
            rack_name: session::RackName::Inserts,
            effect_id: id(LIMITER.id),
        }];
        let (b_verdict, _) = browser.transaction_reply(&edits);
        let (c_verdict, _) = c_abi.transaction(&edits);
        c_abi.drain_reliable();
        assert!(
            matches!(
                b_verdict,
                Verdict::Applied {
                    path: EditPath::Rebuild,
                    ..
                }
            ),
            "{rate} Hz: {b_verdict:?}"
        );
        assert_eq!(b_verdict, c_verdict, "{rate} Hz");
        same_blocks(
            &mut browser,
            &mut c_abi,
            4..8,
            &format!("{rate} Hz floored"),
        );
        let floored = c_latency(&c_abi);
        assert_eq!(
            browser_plan_latency(&mut browser).0,
            floored,
            "{rate} Hz: both hosts report the floored latency"
        );
        assert_eq!(
            browser.host.declare_discontinuity(None),
            host_web::RESULT_OK,
            "{rate} Hz"
        );
        // SAFETY: The session is live and used by this thread alone.
        let code = unsafe { capi::miso_engine_v1_declare_discontinuity(c_abi.session) };
        assert_eq!(code, C_RESULT_OK, "{rate} Hz");
        same_blocks(
            &mut browser,
            &mut c_abi,
            8..16,
            &format!("{rate} Hz declared"),
        );
        assert!(
            c_latency(&c_abi) < floored,
            "{rate} Hz: the declaration resets the floors ({floored})"
        );
        assert_eq!(
            browser_plan_latency(&mut browser).0,
            c_latency(&c_abi),
            "{rate} Hz: both hosts report the reset latency"
        );
    }
}

/// #1688 gate 1 with #1285's floors: a muted track behind a true-peak limiter (and the delay and
/// multiband beside it) sets the plan's latency. A transaction that removes the track and its
/// routes rebuilds the plan, and a fresh compile of the committed session would report a lower
/// latency. The successor is compiled with the predecessor's floors, so both hosts report the same,
/// unchanged latency (and tail) after the swap, and the browser record equals the C ABI's before
/// the swap, at it and after it.
///
/// Test value: red if the browser reports the unfloored latency of a fresh compile, reads a stale
/// plan's row, or encodes the tail differently from the C ABI; and red if the row is vacuous (the
/// latency must be positive and the rebuild must be adopted).
#[test]
fn removing_a_limited_track_keeps_the_plan_latency_alike_on_both_hosts() {
    for rate in RATES {
        let model = model_at(rate);
        let limited = id(LIMITER.track.id);
        let mut removal: Vec<SessionEdit> = model
            .routes
            .iter()
            .filter(|route| {
                matches!(
                    &route.source,
                    session::RouteSource::Track { track_id, .. } if *track_id == limited
                )
            })
            .map(|route| SessionEdit::RemoveRoute {
                route_id: route.id.clone(),
            })
            .collect();
        removal.push(SessionEdit::RemoveTrack { track_id: limited });

        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        let boot = c_abi.plan_latency();
        assert!(boot.0 > 0, "{rate} Hz: the limiter delays the plan");
        assert_eq!(browser_plan_latency(&mut browser), boot, "{rate} Hz: boot");
        for block in 0..4 {
            browser.step(block);
            c_abi.step(block);
        }
        let (b, c) = (browser.transaction(&removal), c_abi.transaction(&removal).0);
        assert_eq!(b, c, "{rate} Hz: one verdict");
        assert!(
            matches!(
                c,
                Verdict::Applied {
                    path: EditPath::Rebuild,
                    ..
                }
            ),
            "{rate} Hz: the removal rebuilds, got {c:?}"
        );
        // Committed, not adopted: both still report the running plan's words.
        assert_eq!(
            browser_plan_latency(&mut browser),
            boot,
            "{rate} Hz: pending"
        );
        assert_eq!(c_abi.plan_latency(), boot, "{rate} Hz: pending");
        for block in 4..8 {
            browser.step(block);
            c_abi.step(block);
        }
        let (watermark, _, _) = c_abi.watermark();
        assert_eq!(
            watermark, c_abi.revision,
            "{rate} Hz: the rebuild is adopted"
        );
        assert_eq!(
            c_abi.plan_latency(),
            boot,
            "{rate} Hz: the floored successor keeps the latency"
        );
        assert_eq!(
            browser_plan_latency(&mut browser),
            boot,
            "{rate} Hz: the browser reports the same, unchanged latency after the swap"
        );

        // Without floors the committed session would report less: the row is not vacuous.
        let mut committed = model.clone();
        committed
            .routes
            .retain(|route| !matches!(
                &route.source,
                session::RouteSource::Track { track_id, .. } if track_id.as_str() == LIMITER.track.id
            ));
        committed
            .tracks
            .retain(|track| track.id.as_str() != LIMITER.track.id);
        let mut fresh = Rig::new(&committed);
        assert!(
            browser_plan_latency(&mut fresh).0 < boot.0,
            "{rate} Hz: a fresh compile of the committed session reports a lower latency"
        );
    }
}

/// #1688 gate 1, the moving row: inserts that add latency to `t1` until its path is the longest
/// make a rebuild that raises the plan's latency. (A strip's `delay_samples` does not: a track
/// delay shifts the output and leaves the PDC report alone, `graph-compiler`'s `track_delay.rs`
/// P2-3.) Both hosts report the boot latency until the adopting block, then the same raised one;
/// the value moved, so the comparison is not vacuous.
///
/// Test value: red if the browser keeps the boot value after a rebuild (a boot-time copy) or
/// reports the candidate's before the adoption block.
#[test]
fn a_rebuild_that_adds_latency_moves_the_plan_latency_alike_on_both_hosts() {
    const LIMITERS: usize = 4;
    for rate in RATES {
        let model = model_at(rate);
        let template = model
            .tracks
            .iter()
            .flat_map(|track| &track.inserts.effects)
            .find(|effect| effect.id.as_str() == LIMITER.id)
            .expect("the session's limiter insert")
            .clone();
        let edits: Vec<SessionEdit> = (0..LIMITERS)
            .map(|index| {
                let mut effect = template.clone();
                effect.id = id(&format!("added-limiter-{index}"));
                SessionEdit::PutTrackEffect {
                    track_id: id(T1.id),
                    rack_name: session::RackName::Inserts,
                    final_position: 2 + index as u32,
                    effect,
                }
            })
            .collect();
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        let boot = c_abi.plan_latency();
        assert_eq!(browser_plan_latency(&mut browser), boot, "{rate} Hz: boot");
        for block in 0..4 {
            browser.step(block);
            c_abi.step(block);
        }
        let (b, c) = (browser.transaction(&edits), c_abi.transaction(&edits).0);
        assert_eq!(b, c, "{rate} Hz: one verdict");
        assert!(
            matches!(
                c,
                Verdict::Applied {
                    path: EditPath::Rebuild,
                    ..
                }
            ),
            "{rate} Hz: the inserts rebuild, got {c:?}"
        );
        assert_eq!(
            browser_plan_latency(&mut browser),
            boot,
            "{rate} Hz: pending"
        );
        assert_eq!(c_abi.plan_latency(), boot, "{rate} Hz: pending");
        browser.step(4);
        c_abi.step(4);
        let moved = c_abi.plan_latency();
        assert!(
            moved.0 > boot.0,
            "{rate} Hz: the added inserts raise the plan latency ({} -> {})",
            boot.0,
            moved.0
        );
        assert_eq!(
            browser_plan_latency(&mut browser),
            moved,
            "{rate} Hz: the browser reports the adopted plan's latency"
        );
    }
}

// ---------------------------------------------------------------------------------------------
// Source seek reports (#1293 D5, D6; batch 15c root ruling)
// ---------------------------------------------------------------------------------------------

impl CAbi {
    fn seek(&self, name: &str, generation: u64, source_frame: u64) {
        // SAFETY: The session is live and used by this thread alone; the name outlives the call.
        let code = unsafe {
            capi::miso_engine_v1_source_seek(
                self.session,
                name.as_ptr(),
                name.len() as u64,
                generation,
                source_frame,
            )
        };
        assert_eq!(code, C_RESULT_OK, "C ABI seek");
    }

    fn seek_at(&self, name: &str, generation: u64, source_frame: u64, anchor: u64) {
        // SAFETY: As `seek`.
        let code = unsafe {
            capi::miso_engine_v1_source_seek_at(
                self.session,
                name.as_ptr(),
                name.len() as u64,
                generation,
                source_frame,
                anchor,
            )
        };
        assert_eq!(code, C_RESULT_OK, "C ABI seek_at");
    }

    /// The source's seek report through `miso_engine_v1_source_seek_report`.
    fn seek_report(&self, name: &str) -> SourceSeekReport {
        let mut out = SourceSeekReport {
            struct_size: SOURCE_SEEK_REPORT_SIZE,
            reserved0: 0,
            generation: 0,
            first_sample: 0,
            source_frame: 0,
            cumulative_underrun_frames: 0,
            cumulative_held_frames: 0,
            reserved: [0; 2],
        };
        // SAFETY: As `seek`; `out` is owned storage of the exact size.
        let code = unsafe {
            capi::miso_engine_v1_source_seek_report(
                self.session,
                name.as_ptr(),
                name.len() as u64,
                &mut out,
            )
        };
        assert_eq!(code, C_RESULT_OK, "C ABI seek report");
        out
    }
}

/// Both hosts' seek report for `name`, equal field by field. Returns
/// `(generation, first_sample, source_frame, underrun, held)`.
fn same_seek_report(
    browser: &mut Rig,
    c_abi: &CAbi,
    name: &str,
    label: &str,
) -> (u64, u64, u64, u64, u64) {
    assert_eq!(
        browser.host.read_seek_report(name.as_bytes()),
        host_web::RESULT_OK,
        "{label}: the browser seek report read"
    );
    let b = *browser.host.seek_report();
    let c = c_abi.seek_report(name);
    assert_eq!(b.struct_size, c.struct_size, "{label}: struct_size");
    assert_eq!(b.reserved0, c.reserved0, "{label}: reserved0");
    assert_eq!(b.generation, c.generation, "{label}: generation");
    assert_eq!(b.first_sample, c.first_sample, "{label}: first_sample");
    assert_eq!(b.source_frame, c.source_frame, "{label}: source_frame");
    assert_eq!(
        b.cumulative_underrun_frames, c.cumulative_underrun_frames,
        "{label}: cumulative underrun frames"
    );
    assert_eq!(
        b.cumulative_held_frames, c.cumulative_held_frames,
        "{label}: cumulative held frames"
    );
    assert_eq!(b.reserved, c.reserved, "{label}: reserved");
    (
        b.generation,
        b.first_sample,
        b.source_frame,
        b.cumulative_underrun_frames,
        b.cumulative_held_frames,
    )
}

/// Block `block` on both hosts with `src-t1` fed generation `generation` from source block
/// `t1_start` (and the other two sources fed at the render position); `None` feeds no `src-t1`
/// chunk this block. The output bits equal.
fn seek_block(
    browser: &mut Rig,
    c_abi: &mut CAbi,
    block: u64,
    generation: u64,
    t1_start: Option<u64>,
    label: &str,
) {
    let rate = browser.rate;
    for (feed, strip) in [T1, T2, T3].into_iter().enumerate() {
        let source = format!("src-{}", strip.id);
        let (chunk_generation, start) = if strip.id == T1.id {
            let Some(t1_start) = t1_start else {
                continue;
            };
            (generation, t1_start)
        } else {
            (1, block)
        };
        let [left, right] = planes(feed as u64, block);
        assert_eq!(
            browser.host.submit_source(
                source.as_bytes(),
                chunk_generation,
                start * u64::from(QUANTUM),
                rate,
                &[&left, &right],
                QUANTUM,
                false,
            ),
            host_web::RESULT_OK,
            "{label}: browser {source}"
        );
        c_abi.submit_from(&source, feed as u64, block, start, chunk_generation);
    }
    assert_eq!(browser.host.render_next(), host_web::RESULT_OK);
    let b = bits(browser.host.output_pcm().expect("browser output"));
    assert_eq!(b, c_abi.render(block), "{label}: block {block} bits");
}

/// #1293 D6 with the batch 15c ruling: after a plain seek, a late anchored seek and an anchored
/// seek ahead, the browser's 64-byte seek report record equals the C ABI's struct field by field,
/// at the call, at every block through the one that applies it, and after, at 48 and 96 kHz.
///
/// Test value: red if either host reports the new generation at the call (before a block has
/// played it), names a different `first_sample` or `source_frame` for the applying block, drops a
/// late anchored seek's lateness from `source_frame` (the C ABI's `first_sample` is the block that
/// played, not the anchor), counts held or underrun frames differently, or leaves a reserved word
/// nonzero. The seeks' own result codes are asserted, and the rows are not vacuous (the new
/// generation is reported with a nonzero source frame, and the late seek lands after its anchor).
#[test]
fn the_seek_report_is_alike_on_both_hosts_for_plain_late_and_future_seeks() {
    const NAME: &str = "src-t1";
    for rate in [48_000, 96_000] {
        let model = model_at(rate);
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        let label = format!("{rate} Hz");
        for block in 0..4 {
            seek_block(&mut browser, &mut c_abi, block, 1, Some(block), &label);
        }
        let boot = same_seek_report(&mut browser, &c_abi, NAME, &label);
        assert_eq!(boot.0, 1, "{label}: generation 1 has rendered");

        // 1. A plain seek: reported at the call as before, then as the block that starts after it.
        let plain_frame = 40_u64;
        let call_block = 4_u64;
        assert_eq!(
            browser
                .host
                .seek_source(NAME.as_bytes(), 2, plain_frame * u64::from(QUANTUM)),
            host_web::RESULT_OK
        );
        c_abi.seek(NAME, 2, plain_frame * u64::from(QUANTUM));
        let at_call = same_seek_report(&mut browser, &c_abi, NAME, &format!("{label} plain: call"));
        assert_eq!(
            at_call.0, 1,
            "{label} plain: the call alone reports generation 1"
        );
        for block in call_block..call_block + 4 {
            seek_block(
                &mut browser,
                &mut c_abi,
                block,
                2,
                Some(plain_frame + (block - call_block)),
                &format!("{label} plain"),
            );
            let report = same_seek_report(
                &mut browser,
                &c_abi,
                NAME,
                &format!("{label} plain: after block {block}"),
            );
            assert_eq!(report.0, 2, "{label} plain: generation 2 in effect");
        }
        let plain = same_seek_report(&mut browser, &c_abi, NAME, &format!("{label} plain: end"));
        assert_eq!(
            plain.1,
            call_block * u64::from(QUANTUM),
            "{label} plain: first block that began after the call"
        );
        assert_eq!(plain.2, plain_frame * u64::from(QUANTUM));

        // 2. A late anchored seek: the anchor (block 3) is behind the render clock (block 8).
        let late_call = call_block + 4;
        let late_frame = 50_u64;
        let anchor = 3 * u64::from(QUANTUM);
        assert_eq!(
            browser.host.seek_source_at(
                NAME.as_bytes(),
                3,
                late_frame * u64::from(QUANTUM),
                anchor
            ),
            host_web::RESULT_OK
        );
        c_abi.seek_at(NAME, 3, late_frame * u64::from(QUANTUM), anchor);
        same_seek_report(&mut browser, &c_abi, NAME, &format!("{label} late: call"));
        for block in late_call..late_call + 4 {
            seek_block(
                &mut browser,
                &mut c_abi,
                block,
                3,
                Some(late_frame + (block - late_call)),
                &format!("{label} late"),
            );
            same_seek_report(
                &mut browser,
                &c_abi,
                NAME,
                &format!("{label} late: after block {block}"),
            );
        }
        let late = same_seek_report(&mut browser, &c_abi, NAME, &format!("{label} late: end"));
        assert_eq!(late.0, 3, "{label} late: generation 3 in effect");
        assert!(
            late.1 > anchor,
            "{label} late: it played after its anchor ({} vs {anchor})",
            late.1
        );

        // 3. An anchored seek two quanta ahead: held until the anchor block.
        let ahead_call = late_call + 4;
        let ahead_frame = 60_u64;
        let ahead_anchor = (ahead_call + 2) * u64::from(QUANTUM);
        assert_eq!(
            browser.host.seek_source_at(
                NAME.as_bytes(),
                4,
                ahead_frame * u64::from(QUANTUM),
                ahead_anchor
            ),
            host_web::RESULT_OK
        );
        c_abi.seek_at(NAME, 4, ahead_frame * u64::from(QUANTUM), ahead_anchor);
        same_seek_report(&mut browser, &c_abi, NAME, &format!("{label} ahead: call"));
        for block in ahead_call..ahead_call + 5 {
            seek_block(
                &mut browser,
                &mut c_abi,
                block,
                4,
                // The ring holds two quanta: feed two, wait for the anchor block to take one.
                match block - ahead_call {
                    0 | 1 => Some(ahead_frame + (block - ahead_call)),
                    2 => None,
                    later => Some(ahead_frame + later - 1),
                },
                &format!("{label} ahead"),
            );
            same_seek_report(
                &mut browser,
                &c_abi,
                NAME,
                &format!("{label} ahead: after block {block}"),
            );
        }
        let ahead = same_seek_report(&mut browser, &c_abi, NAME, &format!("{label} ahead: end"));
        assert_eq!(
            (ahead.0, ahead.1),
            (4, ahead_anchor),
            "{label} ahead: lands at its anchor"
        );
        assert!(ahead.4 > 0, "{label} ahead: the wait held frames");
    }
}

// ---------------------------------------------------------------------------------------------
// Session seek (#1642, #1643 gate 6)
// ---------------------------------------------------------------------------------------------

#[cfg(feature = "test-support")]
impl CAbi {
    fn session_seek(&self, generation: u64, timeline_frame: u64, anchor: Option<u64>) {
        let code = self.session_seek_code(generation, timeline_frame, anchor);
        assert_eq!(code, C_RESULT_OK, "C ABI session seek");
    }

    /// [`Self::session_seek`]'s result code, unchecked.
    fn session_seek_code(&self, generation: u64, timeline_frame: u64, anchor: Option<u64>) -> u32 {
        // SAFETY: The session is live and used by this thread alone.
        unsafe {
            match anchor {
                None => capi::miso_engine_v1_session_seek(self.session, generation, timeline_frame),
                Some(anchor) => capi::miso_engine_v1_session_seek_at(
                    self.session,
                    generation,
                    timeline_frame,
                    anchor,
                ),
            }
        }
    }
}

#[cfg(feature = "test-support")]
/// Block `block` on both hosts: `feeds[i]` is the generation and the first source block of the
/// chunk fed to `src-t(i + 1)`, or `None` for no chunk. The output bits equal.
fn session_block(
    browser: &mut Rig,
    c_abi: &mut CAbi,
    block: u64,
    feeds: [Option<(u64, u64)>; 3],
    label: &str,
) {
    let rate = browser.rate;
    for (feed, strip) in [T1, T2, T3].into_iter().enumerate() {
        let Some((generation, start)) = feeds[feed] else {
            continue;
        };
        let source = format!("src-{}", strip.id);
        let [left, right] = planes(feed as u64, block);
        assert_eq!(
            browser.host.submit_source(
                source.as_bytes(),
                generation,
                start * u64::from(QUANTUM),
                rate,
                &[&left, &right],
                QUANTUM,
                false,
            ),
            host_web::RESULT_OK,
            "{label}: browser {source}"
        );
        c_abi.submit_from(&source, feed as u64, block, start, generation);
    }
    assert_eq!(browser.host.render_next(), host_web::RESULT_OK);
    let b = bits(browser.host.output_pcm().expect("browser output"));
    assert_eq!(b, c_abi.render(block), "{label}: block {block} bits");
}

#[cfg(feature = "test-support")]
/// All three sources' seek reports, equal field by field on both hosts.
fn same_session_reports(
    browser: &mut Rig,
    c_abi: &CAbi,
    label: &str,
) -> [(u64, u64, u64, u64, u64); 3] {
    ["src-t1", "src-t2", "src-t3"].map(|name| same_seek_report(browser, c_abi, name, label))
}

/// #1643 gate 6: a plain session seek and a late anchored session seek, on both hosts at 48 and
/// 96 kHz, over sources of different lengths (`src-t2` is shorter than the other two and clamps
/// to its end). At the call, after every block through the applying one and after, the browser's
/// and the C ABI's seek reports of every source are equal field by field, the output bits are
/// equal, and the browser's timeline (#1641's host-core test reader) reads the frame the long
/// sources' reports name. The C ABI's timeline is not readable from outside the `capi` crate; its
/// own gate (`capi` `runtime::tests::session_seek`) pins it to the same frames, so the row ties
/// the two through the reports both hosts publish.
///
/// Test value: red if either host splits the session seek across blocks (a source reporting the
/// new generation a block before or after another, or before the timeline), clamps the short
/// source differently, or drops a late anchored seek's lateness (the report's `first_sample` is
/// the block that played, after the anchor). The rows are not vacuous: the reports name the new
/// generation, the short source reports its own end, and the late seek lands after its anchor.
#[cfg(feature = "test-support")]
#[test]
fn the_session_seek_is_alike_on_both_hosts_for_plain_and_late_seeks() {
    let quantum = u64::from(QUANTUM);
    for rate in [48_000, 96_000] {
        let mut model = model_at(rate);
        for source in &mut model.sources {
            source.frames = if source.id.as_str() == "src-t2" {
                24 * quantum
            } else {
                64 * quantum
            };
        }
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        let label = format!("{rate} Hz");
        for block in 0..4 {
            session_block(
                &mut browser,
                &mut c_abi,
                block,
                [Some((1, block)); 3],
                &label,
            );
        }
        let boot = same_session_reports(&mut browser, &c_abi, &label);
        assert!(boot.iter().all(|report| report.0 == 1), "{label}: boot");

        // 1. A plain session seek to 40 quanta: the short source clamps to its end (24 quanta).
        let call_block = 4_u64;
        let frame = 40 * quantum;
        assert_eq!(
            browser.host.session_seek(None, 2, frame, None),
            host_web::RESULT_OK
        );
        c_abi.session_seek(2, frame, None);
        let at_call = same_session_reports(&mut browser, &c_abi, &format!("{label} plain: call"));
        assert!(
            at_call.iter().all(|report| report.0 == 1),
            "{label} plain: the call alone reports generation 1"
        );
        for step in 0..4 {
            let block = call_block + step;
            session_block(
                &mut browser,
                &mut c_abi,
                block,
                [Some((2, 40 + step)), None, Some((2, 40 + step))],
                &format!("{label} plain"),
            );
            let reports = same_session_reports(
                &mut browser,
                &c_abi,
                &format!("{label} plain: after block {block}"),
            );
            assert!(
                reports.iter().all(|report| report.0 == 2),
                "{label} plain: generation 2 in effect for every source"
            );
            assert_eq!(
                browser.host.timeline_at_for_test(block * quantum),
                Some((frame + step * quantum) as i64),
                "{label} plain: the timeline in block {block}"
            );
        }
        let plain = same_session_reports(&mut browser, &c_abi, &format!("{label} plain: end"));
        for (index, expected_frame) in [frame, 24 * quantum, frame].into_iter().enumerate() {
            assert_eq!(
                (plain[index].0, plain[index].1),
                (2, call_block * quantum),
                "{label} plain: source {index} landed in the first block after the call"
            );
            assert_eq!(
                plain[index].2, expected_frame,
                "{label} plain: source {index}"
            );
        }

        // 2. A late anchored session seek: its anchor (block 3) is behind the render clock.
        let late_call = call_block + 4;
        let late_frame = 30 * quantum;
        let anchor = 3 * quantum;
        assert_eq!(
            browser.host.session_seek(None, 3, late_frame, Some(anchor)),
            host_web::RESULT_OK
        );
        c_abi.session_seek(3, late_frame, Some(anchor));
        same_session_reports(&mut browser, &c_abi, &format!("{label} late: call"));
        let mut first_late = None;
        for step in 0..4 {
            let block = late_call + step;
            session_block(
                &mut browser,
                &mut c_abi,
                block,
                [Some((3, 30 + step)), None, Some((3, 30 + step))],
                &format!("{label} late"),
            );
            let reports = same_session_reports(
                &mut browser,
                &c_abi,
                &format!("{label} late: after block {block}"),
            );
            let timeline = browser.host.timeline_at_for_test(block * quantum);
            let landed = *first_late.get_or_insert((reports[0].1, reports[0].2, timeline));
            assert_eq!(
                timeline,
                Some(landed.2.expect("a timeline") + (step * quantum) as i64),
                "{label} late: the timeline runs on from where it landed"
            );
        }
        let late = same_session_reports(&mut browser, &c_abi, &format!("{label} late: end"));
        assert!(
            late.iter().all(|report| report.0 == 3),
            "{label} late: generation 3 in effect"
        );
        let (first_sample, source_frame, timeline) = first_late.expect("a late block");
        assert!(
            first_sample > anchor,
            "{label} late: it played after its anchor ({first_sample} vs {anchor})"
        );
        assert_eq!(
            timeline,
            Some(source_frame as i64),
            "{label} late: the timeline and the long source land on the same frame"
        );
        assert_ne!(
            late[1].2, late[0].2,
            "{label} late: the short source clamped, so it lands on another frame"
        );
    }
}

/// Root ruling B16-R9 on both hosts, over #1285's floors (the true-peak limiter insert removed, so
/// every node keeps its latency): an anchored session seek declares nothing, and the floors stay;
/// a refused plain session seek declares nothing, and the floors stay; an accepted plain session
/// seek declares the discontinuity in the same call, so the floors reset, and its seek applies at
/// the next block. At each step both hosts render the same bits and report the same latency and
/// the same seek reports.
///
/// Test value: red if either host's plain seek does not declare (its latency stays floored), if
/// either declares on the anchored form (the latency drops at the anchored call) or before its
/// check (the refused seek resets the floors), or if the two hosts reach different rules (their
/// bits or latencies differ).
#[cfg(feature = "test-support")]
#[test]
fn a_plain_session_seek_declares_and_an_anchored_one_does_not_alike_on_both_hosts() {
    let quantum = u64::from(QUANTUM);
    for rate in [48_000, 96_000] {
        let label = format!("{rate} Hz");
        let model = model_at(rate);
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        same_blocks(&mut browser, &mut c_abi, 0..4, &format!("{label} before"));
        let edits = [SessionEdit::RemoveTrackEffect {
            track_id: id(LIMITER.track.id),
            rack_name: session::RackName::Inserts,
            effect_id: id(LIMITER.id),
        }];
        let (b_verdict, _) = browser.transaction_reply(&edits);
        let (c_verdict, _) = c_abi.transaction(&edits);
        c_abi.drain_reliable();
        assert_eq!(b_verdict, c_verdict, "{label}");
        same_blocks(&mut browser, &mut c_abi, 4..8, &format!("{label} floored"));
        let floored = c_latency(&c_abi);
        assert_eq!(browser_plan_latency(&mut browser).0, floored, "{label}");
        let latencies = |browser: &mut Rig, c_abi: &CAbi, step: &str| {
            let latency = c_latency(c_abi);
            assert_eq!(
                browser_plan_latency(browser).0,
                latency,
                "{label} {step}: both hosts report one latency"
            );
            latency
        };

        // 3. An anchored seek at block 10: nothing is declared, the floors stay.
        let anchor = 10 * quantum;
        assert_eq!(
            browser.host.session_seek(None, 2, 0, Some(anchor)),
            host_web::RESULT_OK
        );
        c_abi.session_seek(2, 0, Some(anchor));
        for block in 8..12 {
            let feed = (block >= 10).then(|| (2, block - 10));
            session_block(&mut browser, &mut c_abi, block, [feed; 3], &label);
        }
        let reports = same_session_reports(&mut browser, &c_abi, &format!("{label} anchored"));
        assert!(
            reports
                .iter()
                .all(|report| (report.0, report.1) == (2, anchor))
        );
        assert_eq!(
            latencies(&mut browser, &c_abi, "anchored"),
            floored,
            "{label}: an anchored session seek keeps the floors"
        );

        // 1. A refused plain seek (generation 2 is stale): nothing is declared.
        assert_eq!(
            browser.host.session_seek(None, 2, 0, None),
            host_web::RESULT_INVALID_ARGUMENT
        );
        assert_eq!(
            c_abi.session_seek_code(2, 0, None),
            capi::RESULT_INVALID_ARGUMENT
        );
        for block in 12..14 {
            session_block(
                &mut browser,
                &mut c_abi,
                block,
                [Some((2, block - 10)); 3],
                &label,
            );
        }
        assert_eq!(
            latencies(&mut browser, &c_abi, "refused"),
            floored,
            "{label}: a refused plain session seek keeps the floors"
        );

        // 2. An accepted plain seek: it declares, the floors reset and the seek applies at the
        // next block.
        let call = 14_u64;
        assert_eq!(
            browser.host.session_seek(None, 3, 0, None),
            host_web::RESULT_OK
        );
        c_abi.session_seek(3, 0, None);
        for block in call..call + 4 {
            session_block(
                &mut browser,
                &mut c_abi,
                block,
                [Some((3, block - call)); 3],
                &label,
            );
        }
        let reports = same_session_reports(&mut browser, &c_abi, &format!("{label} plain"));
        assert!(
            reports
                .iter()
                .all(|report| (report.0, report.1, report.2) == (3, call * quantum, 0)),
            "{label}: the plain seek applied at the next block: {reports:?}"
        );
        assert!(
            latencies(&mut browser, &c_abi, "plain") < floored,
            "{label}: a plain session seek resets the floors ({floored})"
        );
    }
}

/// #1649: [`model_at`] with `t2` riding its fader on both lanes (linear from -3 dB at sample 0 to
/// -15 dB at the run's end, so every grid point moves the lane), VCA `v` holding `t2` and `t3`
/// (`t3` has no automation), and `change` applied.
fn automated_vca_model(rate: u32, change: impl FnOnce(&mut SessionModel)) -> SessionModel {
    let mut model = model_at(rate);
    model.automation.push(session::Automation {
        id: id("t2-fader-ride"),
        target: address(
            Owner::Strip(id(T2.id)),
            Section::Builtins,
            FADER_DB,
            ParameterChannel::Both,
        ),
        segments: vec![session::AutomationSegment {
            shape: session::AutomationShape::Linear,
            start_sample: 0,
            end_sample: u64::from(QUANTUM) * BLOCKS,
            start_value: -3.0,
            end_value: -15.0,
            unit: session::ParameterUnit::Db,
        }],
        offset: session::AutomationOffset::default(),
    });
    model.vcas[0].members = vec![id(T2.id), id(T3.id)];
    change(&mut model);
    model
}

/// A VCA `name` at `db` on both lanes holding `members`.
fn vca_at(name: &str, db: f32, members: &[&str]) -> Vca {
    Vca {
        id: id(name),
        fader: DualMonoFader {
            left_db: db,
            right_db: db,
            left_mute: false,
            right_mute: false,
        },
        members: members.iter().map(|member| id(member)).collect(),
    }
}

/// `SET_CONTROL` of VCA `vca`'s offset on both lanes.
fn vca_offset(vca: &str, db: f32, ramp: Option<u32>) -> SessionEdit {
    SessionEdit::SetControl {
        expected_effect: None,
        address: address(
            Owner::Vca(id(vca)),
            Section::Own,
            VCA_OFFSET_DB,
            ParameterChannel::Both,
        ),
        value: db,
        ramp_samples: ramp,
    }
}

/// The blocks a jump of `jump` samples, the first grid ramp after it (two grid periods of 64) and
/// the plan's latency may still be settling after the edit's block.
fn settle_blocks(browser: &mut Rig, jump: u32) -> u64 {
    let latency = browser_plan_latency(browser).0;
    (latency + u64::from(jump) + 128).div_ceil(u64::from(QUANTUM)) + 1
}

/// One #1649 case on both hosts: `model` plays four blocks, then `edits` commit as one transaction
/// on each host with the same verdict (`path`, one revision) and the same reply bytes. Every block
/// is equal on the two hosts, and from `settle_blocks` after the edit the browser renders the bits
/// of a browser booted on the committed session and fed the same PCM from block 0; when `heard`,
/// that session renders otherwise than `model` there.
fn automated_vca_alike(
    label: &str,
    model: &SessionModel,
    edits: &[SessionEdit],
    path: EditPath,
    jump: u32,
    heard: bool,
) {
    const EDIT: u64 = 4;
    let mut browser = Rig::new(model);
    let mut c_abi = CAbi::new(model);
    same_blocks(&mut browser, &mut c_abi, 0..EDIT, label);
    let (b, b_reply) = browser.transaction_reply(edits);
    let (c, c_reply) = c_abi.transaction(edits);
    assert_eq!(b, c, "{label}: one verdict");
    assert_eq!(b_reply, c_reply, "{label}: one reply");
    assert_eq!(
        b,
        Verdict::Applied {
            path,
            revision: model.revision + 1
        },
        "{label}: the path"
    );
    let committed = snapshot_bytes(|payload, expected| browser.command(payload, expected));
    let mut twin = Rig::boot(&committed, model.sample_rate_hz);
    let mut unedited = Rig::new(model);
    let settle = EDIT + settle_blocks(&mut browser, jump);
    let end = settle + 4;
    assert!(end <= BLOCKS, "{label}: the run fits the sources");
    let mut moved = false;
    for block in 0..end {
        let (t, u) = (twin.step(block), unedited.step(block));
        if block < EDIT {
            continue;
        }
        let (b, c) = (browser.step(block), c_abi.step(block));
        assert_eq!(b, c, "{label}: block {block} alike on both hosts");
        if block >= settle {
            assert_eq!(
                b, t,
                "{label}: block {block} renders the committed session's boot"
            );
            moved |= t != u;
        }
    }
    assert_eq!(moved, heard, "{label}: heard exactly when a value moves");
}

/// #1649 gates 1 and 3 (both hosts). A -6 dB move of `v` over the automated member `t2` (and the
/// plain member `t3`) is `live` on both hosts with the same reply, renders the same bits on both,
/// and from the end of the jump and the first grid ramp after it renders the committed session's
/// boot: with the session's length, and with the edit's own 256 samples (a longer ramp on either
/// member would still differ in that shorter window). A move of `v` to its current value is
/// `model_only` and moves no bit.
///
/// Test value: red if the browser's VCA move does not reach the automated lane's offsets cell
/// (the ride keeps the old composition), if the two hosts differ, or if an unchanged offset
/// retargets the lane.
#[test]
fn a_vca_move_over_an_automated_member_is_live_alike_on_both_hosts() {
    for rate in RATES {
        let model = automated_vca_model(rate, |_| {});
        let fader = model.control_smoothing_samples().fader;
        automated_vca_alike(
            &format!("{rate} Hz, the session's length"),
            &model,
            &[vca_offset(VCA.id, -6.0, None)],
            EditPath::Live,
            fader,
            true,
        );
        automated_vca_alike(
            &format!("{rate} Hz, 256 samples"),
            &model,
            &[vca_offset(VCA.id, -6.0, Some(256))],
            EditPath::Live,
            256,
            true,
        );
        automated_vca_alike(
            &format!("{rate} Hz, the current value"),
            &model,
            &[vca_offset(VCA.id, 0.0, None)],
            EditPath::ModelOnly,
            fader,
            false,
        );
    }
}

/// #1649 gate 2 (both hosts). Ten live moves of `v` before one render give the bits of a twin that
/// made only the last, on both hosts (render reads the newest slot of the offsets cell, whole).
/// Three nested VCAs reach `t2`, declared and nested out of ID order, at 24, -24 and 2e-30 dB in
/// ascending ID order: a move of the third to 1e-30 is `live` and renders the committed session's
/// boot (the `S` bits it writes are the classifier gate's,
/// `a_vca_move_writes_the_automated_lanes_offsets_cell`).
///
/// Test value: red if render applies an older slot than the newest committed one, if a slot mixes
/// two writes, or if a nested reach is composed otherwise than preparation.
#[test]
fn many_vca_moves_before_one_render_are_the_last_alike_on_both_hosts() {
    const EDIT: u64 = 4;
    for rate in RATES {
        let model = automated_vca_model(rate, |_| {});
        let mut many = Rig::new(&model);
        let mut one = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        same_blocks(&mut many, &mut c_abi, 0..EDIT, "before the moves");
        for block in 0..EDIT {
            one.step(block);
        }
        for step in 1..=10 {
            let db = if step == 10 { -6.0 } else { -(step as f32) };
            let edit = [vca_offset(VCA.id, db, None)];
            assert_eq!(many.transaction(&edit), c_abi.transaction(&edit).0);
        }
        one.transaction(&[vca_offset(VCA.id, -6.0, None)]);
        for block in EDIT..EDIT + 24 {
            let (m, c, o) = (many.step(block), c_abi.step(block), one.step(block));
            assert_eq!(m, c, "{rate} Hz: block {block} alike on both hosts");
            assert_eq!(m, o, "{rate} Hz: block {block} is the last move's");
        }

        let nested = automated_vca_model(rate, |model| {
            model.vcas = vec![
                vca_at("vca-c", 2e-30, &[T2.id]),
                vca_at("vca-b", -24.0, &["vca-a"]),
                vca_at("vca-a", 24.0, &["vca-c"]),
            ];
        });
        automated_vca_alike(
            &format!("{rate} Hz, nested"),
            &nested,
            &[vca_offset("vca-c", 1e-30, None)],
            EditPath::Live,
            nested.control_smoothing_samples().fader,
            false,
        );
    }
}

/// #1649 gate 4 (both hosts). Each edit that changes which VCAs reach the automated member `t2`, or
/// how many VCAs the session has, is `live` alike on both hosts and from the end of the jump
/// renders the committed session's boot: `t2` added to a second VCA; that VCA (holding `t2`) made
/// a member of `v`; a new VCA that reaches `t2`; the first VCA of a session that had none; a VCA
/// that reaches `t2` removed.
///
/// Test value: red if any of these rebuilds or differs between the hosts, if a lane no VCA reached
/// had no cell to write when a VCA is added, or if a removed VCA leaves its offset in `S`.
#[test]
fn vca_reach_and_count_changes_over_an_automated_member_are_live_alike_on_both_hosts() {
    let upsert = |vca: Vca| SessionEdit::UpsertVca { vca };
    for rate in RATES {
        let base = automated_vca_model(rate, |model| {
            model.vcas = vec![
                vca_at(VCA.id, -3.0, &[T2.id, T3.id]),
                vca_at("w", -4.0, &[T1.id]),
            ];
        });
        let nested = automated_vca_model(rate, |model| {
            model.vcas = vec![vca_at(VCA.id, -3.0, &[T3.id]), vca_at("w", -4.0, &[T2.id])];
        });
        let bare = automated_vca_model(rate, |model| model.vcas.clear());
        let fader = base.control_smoothing_samples().fader;
        let cases: [(&str, &SessionModel, SessionEdit); 5] = [
            (
                "t2 added to a second VCA",
                &base,
                upsert(vca_at("w", -4.0, &[T1.id, T2.id])),
            ),
            (
                "the VCA holding t2 nested in v",
                &nested,
                upsert(vca_at(VCA.id, -3.0, &[T3.id, "w"])),
            ),
            (
                "a new VCA that reaches t2",
                &base,
                upsert(vca_at("x", -2.0, &[T2.id])),
            ),
            (
                "the first VCA of the session",
                &bare,
                upsert(vca_at(VCA.id, -6.0, &[T2.id, T3.id])),
            ),
            (
                "a VCA that reaches t2 removed",
                &base,
                SessionEdit::RemoveVca { vca_id: id(VCA.id) },
            ),
        ];
        for (label, model, edit) in cases {
            automated_vca_alike(
                &format!("{rate} Hz, {label}"),
                model,
                &[edit],
                EditPath::Live,
                fader,
                true,
            );
        }
    }
}

/// #1650b: `SET_CONTROL` of `t2`'s `fader_db` on `lane` (a hand move on its fader).
#[cfg(feature = "test-support")]
fn t2_fader(lane: ParameterChannel, db: f32) -> SessionEdit {
    SessionEdit::SetControl {
        expected_effect: None,
        address: address(Owner::Strip(id(T2.id)), Section::Builtins, FADER_DB, lane),
        value: db,
        ramp_samples: None,
    }
}

/// One `fader_db` row of `t2` as the control list gives it (#1650 D4): its lane, the bits of its
/// value and its `automated` flag.
type FaderRow = (Option<protocol::ParameterChannel>, u32, bool);

/// The control list's structure epoch and `t2`'s two `fader_db` rows, paged over one host's command
/// entry.
fn t2_fader_rows(
    mut send: impl FnMut(CommandPayload<'_>, ExpectedRevision) -> Vec<u8>,
) -> (u64, Vec<FaderRow>) {
    let mut rows = Vec::new();
    let (mut epoch, mut after) = (0, 0);
    loop {
        let request = ControlListRequest {
            structure_epoch: epoch,
            after_handle: after,
            limit: 256,
            owner: None,
        };
        let bytes = send(
            CommandPayload::ControlListGet(&request),
            ExpectedRevision::Any,
        );
        let mut fields = [0_u16; 4096];
        let DecodedTypedResponseFrame::Success {
            payload: DecodedSuccessResponsePayload::ControlListPage(page),
            ..
        } = ProtocolCodec::default()
            .decode_typed_response(&bytes, &mut DecodeScratch::new(&mut fields))
            .expect("a control page")
        else {
            panic!("expected a control list page")
        };
        rows.extend(page.entries.iter().filter_map(|entry| match &entry.body {
            protocol::ControlBody::Control(row)
                if row.name == "fader_db" && entry.address.owner_id.as_deref() == Some(T2.id) =>
            {
                Some((entry.address.lane, row.value.to_bits(), row.automated))
            }
            _ => None,
        }));
        if page.eof {
            return (page.structure_epoch, rows);
        }
        epoch = page.structure_epoch;
        after = page.last_handle;
    }
}

/// The diagnostic codes of a control list request at `epoch`, or `None` when the host answers it
/// with a page.
fn list_refusal(
    mut send: impl FnMut(CommandPayload<'_>, ExpectedRevision) -> Vec<u8>,
    epoch: u64,
) -> Option<Vec<String>> {
    let request = ControlListRequest {
        structure_epoch: epoch,
        after_handle: 0,
        limit: 1,
        owner: None,
    };
    let bytes = send(
        CommandPayload::ControlListGet(&request),
        ExpectedRevision::Any,
    );
    let mut fields = [0_u16; 4096];
    match ProtocolCodec::default()
        .decode_typed_response(&bytes, &mut DecodeScratch::new(&mut fields))
        .expect("a response")
    {
        DecodedTypedResponseFrame::Success { .. } => None,
        DecodedTypedResponseFrame::NonOk { payload, .. } => Some(
            payload
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.code.clone())
                .collect(),
        ),
    }
}

/// What one host pair gives for one #1650b hand move: the one reply, the committed document, the
/// list after the move and every block from the edit on (equal on both hosts).
#[cfg(feature = "test-support")]
struct HandMove {
    reply: Vec<u8>,
    committed: Vec<u8>,
    rows: (u64, Vec<FaderRow>),
    blocks: Vec<Vec<u32>>,
}

/// `edits` on `model`, after four blocks, as one transaction on both hosts: the same verdict
/// (`live`, one revision) and reply on both, and the same bits of every block up to `end`.
#[cfg(feature = "test-support")]
fn hand_move_alike(label: &str, model: &SessionModel, edits: &[SessionEdit], end: u64) -> HandMove {
    const EDIT: u64 = 4;
    let mut browser = Rig::new(model);
    let mut c_abi = CAbi::new(model);
    same_blocks(&mut browser, &mut c_abi, 0..EDIT, label);
    let (b, reply) = browser.transaction_reply(edits);
    let (c, c_reply) = c_abi.transaction(edits);
    assert_eq!(b, c, "{label}: one verdict");
    assert_eq!(reply, c_reply, "{label}: one reply");
    assert_eq!(
        b,
        Verdict::Applied {
            path: EditPath::Live,
            revision: model.revision + 1
        },
        "{label}: a hand move is live"
    );
    let rows = t2_fader_rows(|payload, expected| browser.command(payload, expected));
    assert_eq!(
        rows,
        t2_fader_rows(|payload, expected| c_abi.send(payload, expected)),
        "{label}: one list"
    );
    let committed = snapshot_bytes(|payload, expected| browser.command(payload, expected));
    let mut blocks = Vec::new();
    for block in EDIT..end {
        let (b, c) = (browser.step(block), c_abi.step(block));
        assert_eq!(b, c, "{label}: block {block} alike on both hosts");
        blocks.push(b);
    }
    HandMove {
        reply,
        committed,
        rows,
        blocks,
    }
}

/// #1650b gates 2 and 5 (both hosts) and root's ruling R17-1. `t2` rides its fader on both lanes
/// (and VCA `v` reaches it). A hand move of `+3` dB on its left lane is `live` alike on both hosts,
/// one revision on the same plan: the committed document stores `+3` as the ride's left offset and
/// keeps the static fader; from the end of the jump the output is, bit for bit, a boot of that
/// document; the list shows both of `t2`'s fader rows `automated`, the left one at `+3` and the
/// right one at `0`, at the same structure epoch. The same transaction prepared by the full path
/// (`control_plane::test_force_full_preparation`) gives the same reply and the same bits of every
/// block, and the in-place leg commits in place on both hosts.
///
/// Test value: red if a hand move on an automated lane writes the static fallback (the curve then
/// overrides it, so the window differs from the boot), rebuilds, reaches the other lane, or if the
/// in-place path (which holds no automation entry unless the entry is an owner of the edit, R17-1)
/// answers or renders otherwise than the full path.
#[cfg(feature = "test-support")]
#[test]
fn a_hand_move_on_an_automated_fader_is_an_offset_alike_on_both_hosts() {
    for rate in RATES {
        let model = automated_vca_model(rate, |_| {});
        let mut expected = automated_vca_model(rate, |model| {
            model.automation[0].offset.left_db = 3.0;
        });
        expected.revision = model.revision + 1;
        let edits = [t2_fader(ParameterChannel::Left, 3.0)];
        let label = format!("{rate} Hz");
        let before = t2_fader_rows(|payload, expected| Rig::new(&model).command(payload, expected));
        assert_eq!(
            before.1,
            vec![
                (
                    Some(protocol::ParameterChannel::Left),
                    0.0_f32.to_bits(),
                    true
                ),
                (
                    Some(protocol::ParameterChannel::Right),
                    0.0_f32.to_bits(),
                    true
                ),
            ],
            "{label}: an automated lane lists its offset, 0"
        );
        let mut probe = Rig::new(&model);
        let settle = 4 + settle_blocks(&mut probe, model.control_smoothing_samples().fader);
        let end = settle + 4;
        assert!(end <= BLOCKS, "{label}: the run fits the sources");

        control_plane::test_take_in_place_counts();
        let in_place = hand_move_alike(&format!("{label}, in place"), &model, &edits, end);
        assert_eq!(
            control_plane::test_take_in_place_counts(),
            (2, 0),
            "{label}: both hosts commit the hand move in place"
        );
        assert_eq!(
            in_place.committed,
            canonical_session_json(&expected)
                .expect("canonical")
                .as_bytes(),
            "{label}: the ride stores +3 on its left lane and the static fader stays"
        );
        assert_eq!(
            in_place.rows,
            (
                before.0,
                vec![
                    (
                        Some(protocol::ParameterChannel::Left),
                        3.0_f32.to_bits(),
                        true
                    ),
                    (
                        Some(protocol::ParameterChannel::Right),
                        0.0_f32.to_bits(),
                        true
                    ),
                ]
            ),
            "{label}: the list reads the offset back, at the same epoch"
        );
        let mut twin = Rig::new(&expected);
        let mut unedited = Rig::new(&model);
        let mut moved = false;
        for block in 0..end {
            let (t, u) = (twin.step(block), unedited.step(block));
            if block >= settle {
                let at = (block - 4) as usize;
                assert_eq!(
                    in_place.blocks[at], t,
                    "{label}: block {block} renders the boot of the stored offset"
                );
                moved |= t != u;
            }
        }
        assert!(moved, "{label}: the offset is heard");

        control_plane::test_force_full_preparation(true);
        let full = hand_move_alike(&format!("{label}, full path"), &model, &edits, end);
        control_plane::test_force_full_preparation(false);
        assert_eq!(
            control_plane::test_take_in_place_counts().0,
            0,
            "{label}: the full leg commits nothing in place"
        );
        assert_eq!(
            full.reply, in_place.reply,
            "{label}: one reply on both paths"
        );
        assert_eq!(full.committed, in_place.committed, "{label}: one document");
        assert_eq!(full.rows, in_place.rows, "{label}: one list");
        assert_eq!(
            full.blocks, in_place.blocks,
            "{label}: the same bits on both paths"
        );
    }
}

/// #1650b gates 4 and 5 (both hosts). On a ride that stores a `+3` dB left offset,
/// `SetAutomationSegments` (a new curve) rebuilds and keeps the offset: the list still reads `+3`,
/// `automated`, and the document keeps it. `RemoveAutomation` rebuilds and removes it with the
/// entry: the lanes return to their static fader, which the list then reads, no longer
/// `automated`, under a new structure epoch, so a list request at the epoch the client held is
/// refused `control.list.stale`. Both hosts answer every step alike and render the same bits.
///
/// Test value: red if a curve edit drops the hand offset (the product reason for owner answer C),
/// if a removed entry leaves its offset or its `automated` flag, or if a client can keep paging a
/// list whose rows changed from offset to static.
#[test]
fn re_authoring_keeps_the_hand_offset_and_removal_ends_it_alike_on_both_hosts() {
    for rate in RATES {
        let model = automated_vca_model(rate, |model| {
            model.automation[0].offset.left_db = 3.0;
        });
        let label = format!("{rate} Hz");
        let ride = model.automation[0].id.clone();
        let static_db = [
            model.tracks[1].fader.left_db,
            model.tracks[1].fader.right_db,
        ];
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        same_blocks(&mut browser, &mut c_abi, 0..3, &label);
        let (epoch, rows) = t2_fader_rows(|payload, expected| browser.command(payload, expected));
        assert_eq!(
            rows,
            vec![
                (
                    Some(protocol::ParameterChannel::Left),
                    3.0_f32.to_bits(),
                    true
                ),
                (
                    Some(protocol::ParameterChannel::Right),
                    0.0_f32.to_bits(),
                    true
                ),
            ],
            "{label}: the stored offset is the row's value"
        );

        let segments = vec![session::AutomationSegment {
            shape: session::AutomationShape::Linear,
            start_sample: 0,
            end_sample: u64::from(QUANTUM) * BLOCKS,
            start_value: -9.0,
            end_value: -1.0,
            unit: session::ParameterUnit::Db,
        }];
        let reauthor = [SessionEdit::SetAutomationSegments {
            automation_id: ride.clone(),
            segments,
        }];
        let (b, b_reply) = browser.transaction_reply(&reauthor);
        let (c, c_reply) = c_abi.transaction(&reauthor);
        assert_eq!(b, c, "{label}: one verdict");
        assert_eq!(
            without_request_id(b_reply),
            without_request_id(c_reply),
            "{label}: one reply"
        );
        assert_eq!(
            b,
            Verdict::Applied {
                path: EditPath::Rebuild,
                revision: model.revision + 1
            },
            "{label}: a curve edit rebuilds"
        );
        same_blocks(
            &mut browser,
            &mut c_abi,
            3..8,
            &format!("{label}: across the swap"),
        );
        let kept = t2_fader_rows(|payload, expected| browser.command(payload, expected));
        assert_eq!(
            kept,
            (epoch, rows.clone()),
            "{label}: the curve edit keeps the offset"
        );
        let document = snapshot_bytes(|payload, expected| browser.command(payload, expected));
        let document = session::parse_session_json(&String::from_utf8(document).expect("utf-8"))
            .expect("the committed document");
        assert_eq!(
            document.automation[0].offset.left_db.to_bits(),
            3.0_f32.to_bits(),
            "{label}: the document keeps the offset"
        );

        let remove = [SessionEdit::RemoveAutomation {
            automation_id: ride,
        }];
        let (b, b_reply) = browser.transaction_reply(&remove);
        let (c, c_reply) = c_abi.transaction(&remove);
        assert_eq!(b, c, "{label}: one verdict");
        assert_eq!(
            without_request_id(b_reply),
            without_request_id(c_reply),
            "{label}: one reply"
        );
        assert_eq!(
            b,
            Verdict::Applied {
                path: EditPath::Rebuild,
                revision: model.revision + 2
            },
            "{label}: a removal rebuilds"
        );
        same_blocks(
            &mut browser,
            &mut c_abi,
            8..14,
            &format!("{label}: after the removal"),
        );
        let (fresh, removed) =
            t2_fader_rows(|payload, expected| browser.command(payload, expected));
        assert_eq!(
            removed,
            vec![
                (
                    Some(protocol::ParameterChannel::Left),
                    static_db[0].to_bits(),
                    false
                ),
                (
                    Some(protocol::ParameterChannel::Right),
                    static_db[1].to_bits(),
                    false
                ),
            ],
            "{label}: the lanes read their static fader again"
        );
        assert!(fresh > epoch, "{label}: the structure epoch advanced");
        let stale = Some(vec!["control.list.stale".to_owned()]);
        assert_eq!(
            list_refusal(
                |payload, expected| browser.command(payload, expected),
                epoch
            ),
            stale,
            "{label}: the browser refuses the old epoch"
        );
        assert_eq!(
            list_refusal(|payload, expected| c_abi.send(payload, expected), epoch),
            stale,
            "{label}: the C ABI refuses the old epoch"
        );
    }
}
