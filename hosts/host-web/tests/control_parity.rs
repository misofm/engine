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
//! the same `live_values_superseded` and `STRIPS_RESTARTED` counts (#1768), and every page of `CONTROL_LIST_GET` and
//! `CONTROL_STATE_GET` and the `SESSION_SNAPSHOT_GET` bytes are byte-identical. The whole run
//! repeats at 44.1, 48, 88.2 and 96 kHz. The script holds the edits that take the rebuild arm (a
//! route into the output and `delay_samples`; the delay's and the multiband compressor's bypasses
//! are live since #1339 and #1340): the browser prepares and publishes the successor like
//! the C ABI does, and the two hosts answer the same bytes and render the same bits across the swap
//! (#1562b). No launch effect declares an audio-rate parameter, so that row of the batch 7
//! amendment has no edit to script.
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
    ENGINE_CONFIG_SIZE, EVENT_LANE_LOSSY, EVENT_LANE_RELIABLE, EngineConfig,
    PLAN_RESOURCE_REPORT_SIZE, PLANAR_OUTPUT_SIZE, Plan, PlanResourceReport, PlanarOutput,
    RESULT_OK as C_RESULT_OK, SOURCE_CHUNK_SIZE, SOURCE_SEEK_REPORT_SIZE, SUBMIT_REPORT_SIZE,
    Session, SourceChunk, SourceSeekReport, SubmitReport,
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
        // mute rebuild the plan; so does a strip's `delay_samples`, which moves the plan's
        // latency. Each is a
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
        // The multiband's and the delay's bypasses are live shunts since #1340 and #1339: a
        // crossfade on both hosts, no swap.
        live(
            34,
            Edit::Bypass {
                insert: MULTIBAND,
                bypassed: true,
            },
        ),
        live(
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
        reserved1: 0,
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
        // One meter stream per strip (#1581 D2): generous for every test session.
        maximum_meter_streams: 4_096,
        maximum_meter_items: 1 << 20,
        maximum_meter_bytes: 1 << 26,
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
        Self::with_limits(model, &limits())
    }

    /// [`Self::new`] under `limits`.
    fn with_limits(model: &SessionModel, limits: &CompileLimits) -> Self {
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
                    limits,
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

    /// The plan's saturating count of revisions completed as a transition fallback, through
    /// `miso_engine_v1_plan_watermark`.
    fn transition_fallback_count(&self) -> u64 {
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
        out.transition_fallback_count
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
        counter(
            |payload| self.send(payload, ExpectedRevision::Any),
            CounterId::LiveValuesSuperseded,
        )
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
        self.submit_planes(
            name,
            generation,
            start_block * u64::from(QUANTUM),
            [&left, &right],
        );
    }

    /// Submit the planes `[left, right]` of `generation` to the source `name` at its frame
    /// `start_frame`; every frame must be accepted.
    fn submit_planes(
        &mut self,
        name: &str,
        generation: u64,
        start_frame: u64,
        planes: [&[f32]; 2],
    ) {
        let frames = u32::try_from(planes[0].len()).expect("a chunk's frames");
        let pointers = [planes[0].as_ptr(), planes[1].as_ptr()];
        let chunk = SourceChunk {
            struct_size: SOURCE_CHUNK_SIZE,
            sample_rate_hz: self.rate,
            generation,
            start_frame,
            planes: pointers.as_ptr(),
            plane_count: 2,
            frames,
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
        assert_eq!(report.accepted_frames, u64::from(frames));
    }

    /// Render block `block` and return the output's bits.
    fn render(&mut self, block: u64) -> Vec<u32> {
        self.render_frames(block * u64::from(QUANTUM), QUANTUM)
    }

    /// Render the `frames` samples from render sample `at` and return the output's bits.
    fn render_frames(&mut self, at: u64, frames: u32) -> Vec<u32> {
        let mut pcm = vec![f32::NAN; frames as usize * 2];
        let output = PlanarOutput {
            struct_size: PLANAR_OUTPUT_SIZE,
            channels: 2,
            samples: pcm.as_mut_ptr(),
            sample_capacity: pcm.len() as u64,
            frames,
            plane_stride_samples: frames,
            reserved: [0; 2],
        };
        // SAFETY: This thread is the plan's only renderer; `output` is owned storage.
        let code = unsafe { capi::miso_engine_v1_render_f32_planar(self.plan, at, &output) };
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
                headers_only: false,
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
            headers_only: false,
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
/// Counter `id`, read with a `COUNTERS_GET` of it alone that `send` submits and answers.
fn counter(send: impl FnOnce(CommandPayload<'_>) -> Vec<u8>, id: CounterId) -> u64 {
    let response = send(CommandPayload::CountersGet(&CountersRequest {
        all: false,
        ids: vec![id as u32],
    }));
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
        .find(|value| value.id == id)
        .expect("the counter is in the snapshot")
        .value
}

fn run(rate: u32, steps: &[Step]) -> (Vec<Vec<u32>>, u64, u64) {
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
    // #1768: the strips the script's rebuilds restarted, counted by the core on both hosts.
    let restarted = counter(
        |payload| c_abi.send(payload, ExpectedRevision::Any),
        CounterId::StripsRestarted,
    );
    assert_eq!(
        counter(
            |payload| browser.command(payload, ExpectedRevision::Any),
            CounterId::StripsRestarted,
        ),
        restarted,
        "{rate} Hz: the browser reports the C ABI's STRIPS_RESTARTED"
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
    (blocks, superseded, restarted)
}

/// The same script on both hosts: every edit ends alike, every block has the same bits, the
/// superseded counts are equal and so are the restarted-strip counts.
///
/// Red if a value lowers to a different lane or cell, ramps over a different length, composes
/// with a VCA or a solo differently, drains differently, or is refused with a different code on
/// one host, or if one host counts its restarted strips itself. At every rate the script also
/// supersedes at least one write (the two fader edits of block 4), its rebuilds restart at least
/// one strip, and at least 40 blocks differ from a run with no edits on the same session, so the
/// script is heard. Which edit moves which block is held in `host-core`'s `live_delta.rs`.
#[test]
fn both_hosts_give_the_same_result_for_the_same_control_edits() {
    let steps = script();
    for rate in RATES {
        let (blocks, superseded, restarted) = run(rate, &steps);
        assert!(
            superseded >= 1,
            "{rate} Hz: the two fader edits of block 4 replace one write"
        );
        assert!(
            restarted >= 1,
            "{rate} Hz: the script's rebuilds restart a strip, so the two hosts compare a count \
             the core moved"
        );
        let (untouched, _, _) = run(rate, &[]);
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

/// #1349's rebuild leg (#1562b): a rebuild, then a live edit before the swap. The status words
/// stay at the initial revision through the blocks before the swap block (neither commit is
/// applied until a plan that carries it renders), then both revisions are in effect together from
/// the swap block's first sample, and the words equal the C ABI's `miso_engine_v1_plan_watermark`
/// after every block.
///
/// Test value: red if the browser publishes a rebuild's revision when it commits rather than when
/// the swap block renders (the words would move before block 3), if the live edit committed behind
/// a pending rebuild is reported at its own earlier block, or if the swap block's sample or
/// outcome flags are the browser's own computation and not the plan's; and (#1294b) red if the
/// status's `blocking_rebuilds` word is not written by the control call that rebuilt.
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
        // #1294b: the rebuild ran on the instance's own thread, so the page's status word counts
        // it at once, before any block renders.
        assert_eq!(browser.host.status().blocking_rebuilds, 1, "{rate} Hz");
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

        // The delay restarts T1: a duck-swap, adopted at its `S` (#1324a).
        let swap = step_both_until_applied(&mut browser, &mut c_abi, 3, model.revision + 2);
        let (revision, first_sample, flags) = status_watermark(&browser);
        assert_eq!(
            revision,
            model.revision + 2,
            "{rate} Hz: both revisions apply"
        );
        assert_eq!(
            first_sample,
            swap * u64::from(QUANTUM),
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
        for block in swap + 1..swap + 5 {
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

/// Blocks from `block` on, on both hosts, equal bit for bit and with one status watermark, until
/// each applies `revision`: a rebuild that restarts a strip is a duck-swap, adopted at its `S`,
/// not at the next block (#1324a). The adoption block.
fn step_both_until_applied(browser: &mut Rig, c_abi: &mut CAbi, block: u64, revision: u64) -> u64 {
    for at in block..block + 64 {
        let (b, c) = (browser.step(at), c_abi.step(at));
        assert_eq!(b, c, "block {at}");
        assert_eq!(status_watermark(browser), c_abi.watermark(), "block {at}");
        if c_abi.watermark().0 >= revision {
            return at;
        }
    }
    panic!("revision {revision} is applied within 64 blocks of block {block}");
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
        let (edited, _, _) = run(rate, &step);
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
/// a mute entry (row 6, which renders since #1651 T3a) rebuilds alike on both.
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
                path: EditPath::Rebuild,
                revision: model.revision + 2
            },
            "{rate} Hz: a mute entry rebuilds (#1651 T3a: the mute row renders)"
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

/// #1711 G1 across hosts (D2): a table-only rebuild of a riding fader while a hand move with its
/// own 4,000 samples still ramps completes exactly on both hosts, as the same rebuild with no move
/// in flight does: the adoption keeps the move's own length, so the B17-R14 transition fallback is
/// deleted. Both hosts take the one decision in the shared control plane: the browser's status
/// words carry the C ABI's watermark, flags included, neither host counts a transition fallback,
/// and both hosts render the same bits.
///
/// Test value: red if either host reports the rebuild differently (an adapter-local rule), or if
/// a rebuild with a level move in flight completes as the transition fallback.
#[test]
fn a_hand_move_in_flight_across_a_table_only_rebuild_completes_exactly_alike_on_both_hosts() {
    let ride = |end_db: f32| SessionEdit::UpsertAutomation {
        automation: session::Automation {
            id: id("t1-fader-ride"),
            target: address(
                Owner::Strip(id(T1.id)),
                Section::Builtins,
                5,
                ParameterChannel::Both,
            ),
            segments: vec![session::AutomationSegment {
                shape: session::AutomationShape::Linear,
                start_sample: 0,
                end_sample: 1_000_000,
                start_value: 0.0,
                end_value: end_db,
                unit: session::ParameterUnit::Db,
            }],
            offset: session::AutomationOffset::default(),
        },
    };
    let hand = CAbi::edits(
        Edit::Fader {
            strip: T1,
            lanes: Lanes::Both,
            db: -6.0,
        },
        Some(4_000),
    );
    for rate in RATES {
        for (label, moved) in [("no move", false), ("a hand move in flight", true)] {
            let model = model_at(rate);
            let mut browser = Rig::new(&model);
            let mut c_abi = CAbi::new(&model);
            let mut block = 0;
            let both = |edits: &[SessionEdit], browser: &mut Rig, c_abi: &mut CAbi| {
                let (b, b_reply) = browser.transaction_reply(edits);
                let (c, c_reply) = c_abi.transaction(edits);
                assert_eq!(b, c, "{rate} Hz, {label}: one verdict");
                assert_eq!(b_reply, c_reply, "{rate} Hz, {label}: one reply");
            };
            both(&[ride(-12.0)], &mut browser, &mut c_abi);
            same_blocks(&mut browser, &mut c_abi, block..block + 4, label);
            block += 4;
            if moved {
                both(&hand, &mut browser, &mut c_abi);
                same_blocks(&mut browser, &mut c_abi, block..block + 3, label);
                block += 3;
            }
            both(&[ride(-6.0)], &mut browser, &mut c_abi);
            same_blocks(&mut browser, &mut c_abi, block..block + 3, label);
            let watermark = c_abi.watermark();
            assert_eq!(
                status_watermark(&browser),
                watermark,
                "{rate} Hz, {label}: the browser status carries the C ABI's watermark"
            );
            assert_eq!(
                watermark.2,
                capi::OUTCOME_EXACT,
                "{rate} Hz, {label}: the rebuild's outcome flags"
            );
            assert_eq!(browser.host.read_watermark(), host_web::RESULT_OK);
            assert_eq!(
                (
                    browser.host.watermark().transition_fallback_count,
                    c_abi.transition_fallback_count()
                ),
                (0, 0),
                "{rate} Hz, {label}: no transition fallback on either host"
            );
        }
    }
}

/// The latency upstream of `t1`'s fader in [`model_at`]'s session, in samples: the console
/// post-insert slot `miso.soft-clip` declares 31 (its descriptor's quality latency); the
/// pre-insert compressor and `t1`'s EQ and compressor inserts declare 0. The fader's node time is
/// render time less it. No host reader reports one strip's upstream latency (the plan reports
/// only the output's, which `t3`'s limiter raises), so it is named here; the bit-equality of
/// [`a_moved_target_defers_its_jump_alike_on_both_hosts`] fails if it is wrong.
const T1_FADER_LATENCY: u64 = 31;

/// #1711 G2 across hosts: on a fader ride flat at -3 dB, a hand move with its own 4,000 samples
/// heard 3 blocks, then a table-only rebuild to -5 dB up to 20,000, then -3 dB (the move's target
/// moves; the rebuild keeps the hand move's stored offset, R17-1): the adoption defers the cell's
/// jump to the move's first free sample on both hosts alike, through the one shared driver. Both
/// hosts give one verdict, reply and committed document per edit, render the same bits in every
/// block, and the rebuild completes exactly on both (`OUTCOME_EXACT`, no transition fallback;
/// #1711 D2). Every block, at the four rates, is bit-equal to a fresh pair whose table has the
/// jump as a real step at the move's first free sample in the fader's node time (render time
/// 4,512 less [`T1_FADER_LATENCY`]), given the same move and, at the same block, a table-only
/// rebuild that moves nothing (a split at 950,000), both keeping the stored offset.
///
/// Test value: red if either host adopts the moved target differently (an adapter-local rule, or
/// a browser successor that does not carry the stage's ramp the deferral waits for), or if the
/// shared adoption does not defer the jump to the move's first free sample (it lands at the swap,
/// fires at another time, or the grid cuts the move short).
#[test]
fn a_moved_target_defers_its_jump_alike_on_both_hosts() {
    // The move's first free sample in the fader's node time: its block-4 entry in render time plus
    // its 4,000 samples, less the latency upstream of `t1`'s fader.
    const FREE: u64 = 4 * QUANTUM as u64 + 4_000 - T1_FADER_LATENCY;
    let ride = |pieces: &[(u64, u64, f32)], offset| SessionEdit::UpsertAutomation {
        automation: session::Automation {
            id: id("t1-fader-ride"),
            target: address(
                Owner::Strip(id(T1.id)),
                Section::Builtins,
                5,
                ParameterChannel::Both,
            ),
            segments: pieces
                .iter()
                .map(|&(start, end, db)| session::AutomationSegment {
                    shape: session::AutomationShape::Step,
                    start_sample: start,
                    end_sample: end,
                    start_value: db,
                    end_value: db,
                    unit: session::ParameterUnit::Db,
                })
                .collect(),
            offset,
        },
    };
    let none = session::AutomationOffset::default();
    let hand = CAbi::edits(
        Edit::Fader {
            strip: T1,
            lanes: Lanes::Both,
            db: -6.0,
        },
        Some(4_000),
    );
    for rate in RATES {
        let model = model_at(rate);
        let (mut browser, mut c_abi) = (Rig::new(&model), CAbi::new(&model));
        let (mut browser_r, mut c_abi_r) = (Rig::new(&model), CAbi::new(&model));
        let reference = |split: u64, offset| {
            ride(
                &[
                    (0, FREE, -3.0),
                    (FREE, 20_000, -5.0),
                    (20_000, split, -3.0),
                    (split, 1_000_000, -3.0),
                ],
                offset,
            )
        };
        let both = |edits: &[SessionEdit], browser: &mut Rig, c_abi: &mut CAbi, label: &str| {
            let (b, b_reply) = browser.transaction_reply(edits);
            let (c, c_reply) = c_abi.transaction(edits);
            assert_eq!(b, c, "{rate} Hz, {label}: one verdict");
            assert_eq!(b_reply, c_reply, "{rate} Hz, {label}: one reply");
        };
        both(
            &[ride(&[(0, 1_000_000, -3.0)], none)],
            &mut browser,
            &mut c_abi,
            "the flat ride",
        );
        both(
            &[reference(900_000, none)],
            &mut browser_r,
            &mut c_abi_r,
            "the reference ride",
        );
        for block in 0..60 {
            if block == 4 {
                both(&hand, &mut browser, &mut c_abi, "the hand move");
                both(&hand, &mut browser_r, &mut c_abi_r, "the reference move");
            }
            if block == 7 {
                let committed =
                    snapshot_bytes(|payload, expected| browser.command(payload, expected));
                assert_eq!(
                    committed,
                    snapshot_bytes(|payload, expected| c_abi.send(payload, expected)),
                    "{rate} Hz: one committed document"
                );
                let committed =
                    session::parse_session_json(core::str::from_utf8(&committed).expect("utf-8"))
                        .expect("the committed document");
                let offset = committed
                    .automation
                    .iter()
                    .find(|entry| entry.id == id("t1-fader-ride"))
                    .expect("the ride")
                    .offset;
                assert_ne!(offset, none, "{rate} Hz: the hand move is stored");
                both(
                    &[ride(
                        &[(0, 20_000, -5.0), (20_000, 1_000_000, -3.0)],
                        offset,
                    )],
                    &mut browser,
                    &mut c_abi,
                    "the rebuild",
                );
                both(
                    &[reference(950_000, offset)],
                    &mut browser_r,
                    &mut c_abi_r,
                    "the reference rebuild",
                );
            }
            let (b, c) = (browser.step(block), c_abi.step(block));
            assert_eq!(b, c, "{rate} Hz: block {block}");
            let (b_r, c_r) = (browser_r.step(block), c_abi_r.step(block));
            assert_eq!(b_r, c_r, "{rate} Hz: block {block}, the reference");
            assert_eq!(b, b_r, "{rate} Hz: block {block} against the reference");
        }
        let watermark = c_abi.watermark();
        assert_eq!(
            status_watermark(&browser),
            watermark,
            "{rate} Hz: the browser status carries the C ABI's watermark"
        );
        assert_eq!(
            watermark.2,
            capi::OUTCOME_EXACT,
            "{rate} Hz: the rebuild's outcome flags"
        );
        assert_eq!(browser.host.read_watermark(), host_web::RESULT_OK);
        assert_eq!(
            (
                browser.host.watermark().transition_fallback_count,
                c_abi.transition_fallback_count()
            ),
            (0, 0),
            "{rate} Hz: no transition fallback on either host"
        );
    }
}

/// #1655 U3 fold 1 (MAJOR-1) across hosts: the composed twin of
/// `a_table_step_inside_a_deferred_moves_ramp_lands_alike_on_both_hosts` (deleted by TA5). `t1` is also in VCA
/// `v`, whose `offset_db` rides linearly from 0 to -6 dB over 48,000 samples in every plan, so
/// `t1`'s fader channels are composed lanes over its own ride and the VCA's. The adoption holds
/// and defers every cursor; the own table's step at 2,000 replaces the move's ramp on the whole
/// lane, as one cell's step does. Both hosts render the same bits as each other and as the fresh
/// pair with the same VCA ride, at the four rates, `EXACT` with no transition fallback.
///
/// Test value: red if a composed lane's jump drops only the stepping cursor's deferral and hold
/// (the VCA cursor then freezes the lane until 4,512 while the fresh pair follows the ride), on
/// either host.
#[test]
fn a_table_step_inside_a_deferred_moves_ramp_releases_a_composed_lane_alike_on_both_hosts() {
    table_step_inside_a_deferred_move(|rate| {
        let mut model = model_at(rate);
        model.vcas = vec![vca_at(VCA.id, 0.0, &[T1.id])];
        model.automation.push(vca_entry(
            VCA.id,
            VCA_OFFSET_DB,
            vec![vca_piece(
                session::AutomationShape::Linear,
                (0, 48_000),
                (0.0, -6.0),
                session::ParameterUnit::Db,
            )],
        ));
        model
    });
}

/// The table step inside a deferred move's ramp (#1711 U1 fold 1), over the session
/// `model_of(rate)`.
fn table_step_inside_a_deferred_move(model_of: fn(u32) -> SessionModel) {
    let segment = |shape, start, end, from, to| session::AutomationSegment {
        shape,
        start_sample: start,
        end_sample: end,
        start_value: from,
        end_value: to,
        unit: session::ParameterUnit::Db,
    };
    let ride = |segments: Vec<session::AutomationSegment>, offset| SessionEdit::UpsertAutomation {
        automation: session::Automation {
            id: id("t1-fader-ride"),
            target: address(
                Owner::Strip(id(T1.id)),
                Section::Builtins,
                5,
                ParameterChannel::Both,
            ),
            segments,
            offset,
        },
    };
    let (step, linear) = (
        session::AutomationShape::Step,
        session::AutomationShape::Linear,
    );
    let stepped = |first: f32, split: u64, offset| {
        ride(
            vec![
                segment(step, 0, 2_000, first, first),
                segment(linear, 2_000, 8_000, -8.0, -14.0),
                segment(step, 8_000, split, -14.0, -14.0),
                segment(step, split, 1_000_000, -14.0, -14.0),
            ],
            offset,
        )
    };
    let none = session::AutomationOffset::default();
    let hand = CAbi::edits(
        Edit::Fader {
            strip: T1,
            lanes: Lanes::Both,
            db: -6.0,
        },
        Some(4_000),
    );
    for rate in RATES {
        let model = model_of(rate);
        let (mut browser, mut c_abi) = (Rig::new(&model), CAbi::new(&model));
        let (mut browser_r, mut c_abi_r) = (Rig::new(&model), CAbi::new(&model));
        let both = |edits: &[SessionEdit], browser: &mut Rig, c_abi: &mut CAbi, label: &str| {
            let (b, b_reply) = browser.transaction_reply(edits);
            let (c, c_reply) = c_abi.transaction(edits);
            assert_eq!(b, c, "{rate} Hz, {label}: one verdict");
            assert_eq!(b_reply, c_reply, "{rate} Hz, {label}: one reply");
        };
        both(
            &[ride(vec![segment(step, 0, 1_000_000, -3.0, -3.0)], none)],
            &mut browser,
            &mut c_abi,
            "the flat ride",
        );
        both(
            &[stepped(-3.0, 900_000, none)],
            &mut browser_r,
            &mut c_abi_r,
            "the reference ride",
        );
        for block in 0..60 {
            if block == 4 {
                both(&hand, &mut browser, &mut c_abi, "the hand move");
                both(&hand, &mut browser_r, &mut c_abi_r, "the reference move");
            }
            if block == 7 {
                // A table-only rebuild keeps the hand move's stored offset (R17-1).
                let committed =
                    snapshot_bytes(|payload, expected| browser.command(payload, expected));
                assert_eq!(
                    committed,
                    snapshot_bytes(|payload, expected| c_abi.send(payload, expected)),
                    "{rate} Hz: one committed document"
                );
                let committed =
                    session::parse_session_json(core::str::from_utf8(&committed).expect("utf-8"))
                        .expect("the committed document");
                let offset = committed
                    .automation
                    .iter()
                    .find(|entry| entry.id == id("t1-fader-ride"))
                    .expect("the ride")
                    .offset;
                both(
                    &[stepped(-5.0, 900_000, offset)],
                    &mut browser,
                    &mut c_abi,
                    "the rebuild",
                );
                both(
                    &[stepped(-3.0, 950_000, offset)],
                    &mut browser_r,
                    &mut c_abi_r,
                    "the reference rebuild",
                );
            }
            let (b, c) = (browser.step(block), c_abi.step(block));
            assert_eq!(b, c, "{rate} Hz: block {block}");
            let (b_r, c_r) = (browser_r.step(block), c_abi_r.step(block));
            assert_eq!(b_r, c_r, "{rate} Hz: block {block}, the reference");
            assert_eq!(b, b_r, "{rate} Hz: block {block} against the reference");
        }
        let watermark = c_abi.watermark();
        assert_eq!(
            status_watermark(&browser),
            watermark,
            "{rate} Hz: the browser status carries the C ABI's watermark"
        );
        assert_eq!(watermark.2, capi::OUTCOME_EXACT, "{rate} Hz: outcome flags");
        assert_eq!(browser.host.read_watermark(), host_web::RESULT_OK);
        assert_eq!(
            (
                browser.host.watermark().transition_fallback_count,
                c_abi.transition_fallback_count()
            ),
            (0, 0),
            "{rate} Hz: no transition fallback on either host"
        );
    }
}

/// #1651 gate 8 across hosts (U1 fold 1 NIT of #1711, restored without its B17-R14 window
/// claim): in a mute-only session (a stored mute curve on `t1`, no fader entry), a rebuild that
/// changes the mute curve while a hand fader move with its own 4,000 samples still ramps. Both
/// hosts give one verdict and reply per edit, render the same bits in every block and report the
/// same watermark.
///
/// Test value: red if either host adopts a mute-curve rebuild under a ramping level move
/// differently (an adapter-local adoption, or a browser successor that does not carry the
/// fader stage's ramp or the mute cell's state as the C ABI's does).
#[test]
fn a_mute_curve_rebuild_while_a_level_move_ramps_renders_alike_on_both_hosts() {
    let curve = |close: u64| SessionEdit::UpsertAutomation {
        automation: session::Automation {
            id: id("t1-mute-curve"),
            target: address(
                Owner::Strip(id(T1.id)),
                Section::Builtins,
                6,
                ParameterChannel::Both,
            ),
            segments: [(0, close, 0.0), (close, 1_000_000, 1.0)]
                .iter()
                .map(|&(start, end, value)| session::AutomationSegment {
                    shape: session::AutomationShape::Step,
                    start_sample: start,
                    end_sample: end,
                    start_value: value,
                    end_value: value,
                    unit: session::ParameterUnit::Linear,
                })
                .collect(),
            offset: session::AutomationOffset::default(),
        },
    };
    let hand = CAbi::edits(
        Edit::Fader {
            strip: T1,
            lanes: Lanes::Both,
            db: -6.0,
        },
        Some(4_000),
    );
    for rate in RATES {
        let model = model_at(rate);
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        let both = |edits: &[SessionEdit], browser: &mut Rig, c_abi: &mut CAbi| {
            let (b, b_reply) = browser.transaction_reply(edits);
            let (c, c_reply) = c_abi.transaction(edits);
            assert_eq!(b, c, "{rate} Hz: one verdict");
            assert_eq!(b_reply, c_reply, "{rate} Hz: one reply");
            b
        };
        let rebuilt = |verdict: Verdict| {
            assert!(
                matches!(
                    verdict,
                    Verdict::Applied {
                        path: EditPath::Rebuild,
                        ..
                    }
                ),
                "{rate} Hz: a mute curve edit rebuilds: {verdict:?}"
            );
        };
        rebuilt(both(&[curve(20_000)], &mut browser, &mut c_abi));
        same_blocks(&mut browser, &mut c_abi, 0..4, "the mute-only session");
        both(&hand, &mut browser, &mut c_abi);
        same_blocks(&mut browser, &mut c_abi, 4..7, "the hand move");
        rebuilt(both(&[curve(30_000)], &mut browser, &mut c_abi));
        same_blocks(&mut browser, &mut c_abi, 7..40, "the mute curve rebuild");
        assert_eq!(
            status_watermark(&browser),
            c_abi.watermark(),
            "{rate} Hz: the browser status carries the C ABI's watermark"
        );
    }
}

/// #1711 G4 across hosts (A1): an exact hand unmute that frees a lane held over a moving ride
/// discharges the move the hold owed, alike on both hosts. `t1` rides linearly from 0 to -6 dB
/// over `[0, 1,280)`, holds -6 dB to 2,320, then rides on to -12 dB. A hand mute (ramp 256) at
/// block 4 and an exact hand unmute (ramp 0) at block 8 hold the lane over moving grid points; a
/// hand mute (256) at block 12 and a hand unmute (200) at block 16 hold it again where the ride is
/// flat (nothing is remembered there; the hold ends at 2,248). Both hosts give one verdict and
/// reply per edit, the same watermark (`EXACT`, no transition fallback) and the same bits in
/// every block, and from block 24 on every block equals the pair of rigs that took only the
/// second hold, at the four rates. Block 24 is past the first hold's own difference at every
/// rate (measured: equal from block 12, 13, 21 and 22 at 44.1, 48, 88.2 and 96 kHz), and inside
/// the difference a stale obligation makes (measured once with the line deleted: blocks 21 to 28
/// at 44.1 kHz up to 25 to 40 at 96 kHz).
///
/// Test value: red if the free write at the unmute's entry does not discharge the owed move (the
/// `owed = false` of `HeldRampObligation::block`, through the browser's control path as through
/// the C ABI's): the stale obligation lands a move over `J` at the second hold's end.
#[test]
fn an_exact_unmute_discharges_the_owed_move_alike_on_both_hosts() {
    let linear = |start: u64, end: u64, from: f32, to: f32| session::AutomationSegment {
        shape: session::AutomationShape::Linear,
        start_sample: start,
        end_sample: end,
        start_value: from,
        end_value: to,
        unit: session::ParameterUnit::Db,
    };
    let ride = SessionEdit::UpsertAutomation {
        automation: session::Automation {
            id: id("t1-fader-ride"),
            target: address(
                Owner::Strip(id(T1.id)),
                Section::Builtins,
                5,
                ParameterChannel::Both,
            ),
            segments: vec![
                linear(0, 1_280, 0.0, -6.0),
                linear(1_280, 2_320, -6.0, -6.0),
                linear(2_320, 1_000_000, -6.0, -12.0),
            ],
            offset: session::AutomationOffset::default(),
        },
    };
    let mute = |muted: bool, ramp: u32| {
        CAbi::edits(
            Edit::Mute {
                strip: T1,
                lanes: Lanes::Both,
                muted,
            },
            Some(ramp),
        )
    };
    let first_hold = [(4, mute(true, 256)), (8, mute(false, 0))];
    let second_hold = [(12, mute(true, 256)), (16, mute(false, 200))];
    for rate in RATES {
        let model = model_at(rate);
        let (mut browser, mut c_abi) = (Rig::new(&model), CAbi::new(&model));
        let (mut browser_r, mut c_abi_r) = (Rig::new(&model), CAbi::new(&model));
        let both = |edits: &[SessionEdit], browser: &mut Rig, c_abi: &mut CAbi, label: &str| {
            let (b, b_reply) = browser.transaction_reply(edits);
            let (c, c_reply) = c_abi.transaction(edits);
            assert_eq!(b, c, "{rate} Hz, {label}: one verdict");
            assert_eq!(b_reply, c_reply, "{rate} Hz, {label}: one reply");
        };
        both(
            std::slice::from_ref(&ride),
            &mut browser,
            &mut c_abi,
            "the ride",
        );
        both(
            std::slice::from_ref(&ride),
            &mut browser_r,
            &mut c_abi_r,
            "the ride, reference",
        );
        for block in 0..48 {
            for (at, edits) in first_hold.iter().chain(&second_hold) {
                if *at == block {
                    both(edits, &mut browser, &mut c_abi, "both holds");
                }
            }
            for (at, edits) in &second_hold {
                if *at == block {
                    both(edits, &mut browser_r, &mut c_abi_r, "the second hold");
                }
            }
            let (b, c) = (browser.step(block), c_abi.step(block));
            assert_eq!(b, c, "{rate} Hz: block {block}, both holds");
            let (b_r, c_r) = (browser_r.step(block), c_abi_r.step(block));
            assert_eq!(b_r, c_r, "{rate} Hz: block {block}, the second hold");
            if block >= 24 {
                assert_eq!(
                    b, b_r,
                    "{rate} Hz: block {block} against the second hold alone"
                );
            }
        }
        let watermark = c_abi.watermark();
        assert_eq!(
            status_watermark(&browser),
            watermark,
            "{rate} Hz: the browser status carries the C ABI's watermark"
        );
        assert_eq!(watermark.2, capi::OUTCOME_EXACT, "{rate} Hz: outcome flags");
        assert_eq!(browser.host.read_watermark(), host_web::RESULT_OK);
        assert_eq!(
            (
                browser.host.watermark().transition_fallback_count,
                c_abi.transition_fallback_count()
            ),
            (0, 0),
            "{rate} Hz: no transition fallback on either host"
        );
    }
}

/// #1651 gates 1 and 3 across hosts (U1 fold 1, MINOR-4): the strip's stored mute under
/// composition renders the same bits on the browser and the C ABI. `t1`'s mute curve closes at
/// 2,000 and opens at 3,500 on both lanes, and VCA `m` holds `t1`. Each of solo on `t2`, a VCA
/// mute of `t1` and a hand mute of `t1` is set at block 2 (the curve open), then released while
/// the curve is closed, released while it is open, or kept; after the curve's opening a plain
/// session seek to timeline 2,600 lands inside the muted span and the curve opens again 900
/// samples later. Every transaction gets the same verdict and reply bytes on both hosts and
/// every block the same bits, at quanta 128 and 37 (a quantum that puts the steps, the terms and
/// the seek off every grid and block boundary). The curve is heard: `t1`'s muted span differs from
/// a twin with no curve.
///
/// Test value: red if a host adapter delivers the seek or a term edit differently (both hosts run
/// the one core, so the unique catch is the adapter: a step, a term or the seek's exact set landing
/// at another sample or over another ramp, or a different answer to a term edit). The send follows
/// `t1`'s mute, so the following route's cell is in the comparison too.
#[cfg(feature = "test-support")]
#[test]
fn a_composed_stored_mute_renders_alike_on_both_hosts() {
    let curve = || session::Automation {
        id: id("t1-mute-curve"),
        target: address(
            Owner::Strip(id(T1.id)),
            Section::Builtins,
            MUTE,
            ParameterChannel::Both,
        ),
        segments: [
            (0, 2_000, 0.0),
            (2_000, 3_500, 1.0),
            (3_500, 1_000_000, 0.0),
        ]
        .map(
            |(start_sample, end_sample, value)| session::AutomationSegment {
                shape: session::AutomationShape::Step,
                start_sample,
                end_sample,
                start_value: value,
                end_value: value,
                unit: session::ParameterUnit::Linear,
            },
        )
        .to_vec(),
        offset: session::AutomationOffset::default(),
    };
    const VCA_M: VcaRef = VcaRef { id: "m" };
    let rate = 48_000;
    for quantum in [128_u32, 37] {
        let q = u64::from(quantum);
        let mut model = model_at(rate);
        model.quantum_frames = quantum;
        model.vcas.push(Vca {
            id: id(VCA_M.id),
            fader: DualMonoFader {
                left_db: 0.0,
                right_db: 0.0,
                left_mute: false,
                right_mute: false,
            },
            members: vec![id(T1.id)],
        });
        let plain = model.clone();
        model.automation.push(curve());
        // The block that holds a sample, and the blocks of the script.
        let block_of = |sample: u64| sample.div_ceil(q);
        let (set_at, seek_at) = (2, block_of(4_200));
        let blocks = seek_at + block_of(1_900);
        let terms = [
            (
                "solo",
                Edit::Solo {
                    strip: T2,
                    on: true,
                },
                Edit::Solo {
                    strip: T2,
                    on: false,
                },
            ),
            (
                "vca",
                Edit::VcaMute {
                    vca: VCA_M,
                    lanes: Lanes::Both,
                    muted: true,
                },
                Edit::VcaMute {
                    vca: VCA_M,
                    lanes: Lanes::Both,
                    muted: false,
                },
            ),
            (
                "hand",
                Edit::Mute {
                    strip: T1,
                    lanes: Lanes::Both,
                    muted: true,
                },
                Edit::Mute {
                    strip: T1,
                    lanes: Lanes::Both,
                    muted: false,
                },
            ),
        ];
        for (term, on, off) in terms {
            for release in [Some(block_of(2_600)), Some(block_of(1_000)), None] {
                let label = format!("quantum {quantum}, {term}, released at block {release:?}");
                let document = canonical_session_json(&model).expect("canonical session");
                let mut browser = Rig::boot_at(document.as_bytes(), rate, quantum);
                let mut c_abi = CAbi::with_limits(
                    &model,
                    &CompileLimits {
                        source_ring_frames: quantum * 2,
                        ..limits()
                    },
                );
                let plain_document = canonical_session_json(&plain).expect("canonical session");
                let mut twin = Rig::boot_at(plain_document.as_bytes(), rate, quantum);
                let (mut generation, mut timeline) = (1, 0);
                let mut heard = false;
                for block in 0..blocks {
                    let edit = if block == set_at {
                        Some(on)
                    } else if Some(block) == release {
                        Some(off)
                    } else {
                        None
                    };
                    if let Some(edit) = edit {
                        let edits = CAbi::edits(edit, None);
                        let (b, b_reply) = browser.transaction_reply(&edits);
                        let (c, c_reply) = c_abi.transaction(&edits);
                        assert_eq!(b, c, "{label}: one verdict at block {block}");
                        assert_eq!(b_reply, c_reply, "{label}: one reply at block {block}");
                        assert!(
                            matches!(b, Verdict::Applied { .. }),
                            "{label}: the term at block {block} commits: {b:?}"
                        );
                    }
                    if block == seek_at {
                        generation += 1;
                        timeline = 2_600;
                        assert_eq!(
                            browser.host.session_seek(generation, timeline, None),
                            host_web::RESULT_OK
                        );
                        assert_eq!(
                            twin.host.session_seek(generation, timeline, None),
                            host_web::RESULT_OK
                        );
                        c_abi.session_seek(generation, timeline, None);
                    }
                    let mut outputs = Vec::new();
                    for leg in [&mut browser, &mut twin] {
                        for (feed, strip) in [T1, T2, T3].into_iter().enumerate() {
                            let [left, right] = [0, 1].map(|lane| {
                                (timeline..timeline + q)
                                    .map(|frame| sample(feed as u64, lane, frame))
                                    .collect::<Vec<f32>>()
                            });
                            assert_eq!(
                                leg.host.submit_source(
                                    format!("src-{}", strip.id).as_bytes(),
                                    generation,
                                    timeline,
                                    rate,
                                    &[&left, &right],
                                    quantum,
                                    false,
                                ),
                                host_web::RESULT_OK,
                                "{label}: browser source {}",
                                strip.id
                            );
                            if outputs.is_empty() {
                                c_abi.submit_planes(
                                    &format!("src-{}", strip.id),
                                    generation,
                                    timeline,
                                    [&left, &right],
                                );
                            }
                        }
                        assert_eq!(leg.host.render_next(), host_web::RESULT_OK);
                        outputs.push(bits(leg.host.output_pcm().expect("browser output")));
                    }
                    let c = c_abi.render_frames(block * q, quantum);
                    assert_eq!(outputs[0], c, "{label}: block {block} bits");
                    // Inside the curve's muted span, after its ramp: `t1` is muted there,
                    // unlike in the twin with no curve and no term.
                    if (2_600..3_400).contains(&timeline) && outputs[0] != outputs[1] {
                        heard = true;
                    }
                    timeline += q;
                }
                assert!(heard, "{label}: the curve's muted span is heard");
            }
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
/// #1690b gate 1: after the rebuild both arm `t1`'s compressor tap and play the same loud PCM; the
/// C ABI configures `[{master, gain reduction}]`, one batch per meter window. After each later
/// step both play [`MASTER_PLAY_BLOCKS`] blocks, a whole number of either host's meter windows,
/// and the browser's `masterGrDb` (its lease frame's master word, present with
/// `master_gr_present`) and the C ABI's last master record (`VALID` or not) are the same bits and
/// present together: the compressor's reduction, nonzero, while `t1` is the master; absent once the
/// master moves to `bus` (no armed tap) and once it is cleared.
///
/// Test value: red if one host lacks the row, refuses a designation the other takes, spends a
/// revision on it, lists another master, holds two masters at once, saves the designation into
/// the snapshot, or admits a strip the session does not have; or if either host computes the
/// master's reading its own way, reads another window, or reports `0` where the other reports
/// absent (#1690b).
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
    // #1690b: long enough for every step's play.
    source.frames = 1 << 20;
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
    // #1690b: the browser leg boots with a meter window and observation capacity, so it grants
    // the meter lease and arms the tap.
    browser.host = host_web::AudioWorkletEngineHost::boot(
        canonical_session_json(&model)
            .expect("canonical session")
            .as_bytes(),
        host_web::WebBootOptions {
            require_sample_rate_hz: rate,
            require_quantum_frames: QUANTUM,
            source_ring_frames: QUANTUM * 2,
            live_control_meter_blocks: host_web::DEFAULT_METER_BLOCKS.into(),
            ..host_web::WebBootOptions::explicit_defaults()
        },
    )
    .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.diagnostic())));
    let mut c_abi = CAbi::new(&model);
    let mut block = 0;

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
        let _ = c_abi.render(block);
        block += 1;
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
        // #1690b gate 1: the master's gain reduction, on each host.
        if step == 0 {
            arm_master_reading(&mut browser, &mut c_abi);
            continue;
        }
        // Each step plays the stem from its frame 0, in a new generation, from the next block.
        let generation = step as u64 + 1;
        assert_eq!(
            browser.host.seek_source(b"stem", generation, 0),
            host_web::RESULT_OK
        );
        // SAFETY: The session is live and used by this thread alone; the name outlives the call.
        let code = unsafe {
            capi::miso_engine_v1_source_seek(c_abi.session, b"stem".as_ptr(), 4, generation, 0)
        };
        assert_eq!(code, C_RESULT_OK, "C ABI seek");
        let mut readings = None;
        for fed in 0..MASTER_PLAY_BLOCKS {
            let [left, right] = planes(0, block);
            assert_eq!(
                browser.host.submit_source(
                    b"stem",
                    generation,
                    fed * u64::from(QUANTUM),
                    rate,
                    &[&left, &right],
                    QUANTUM,
                    false,
                ),
                host_web::RESULT_OK,
                "step {step}: the browser takes the stem's PCM: {}",
                String::from_utf8_lossy(browser.host.diagnostic())
            );
            c_abi.submit_from("stem", 0, block, fed, generation);
            assert_eq!(browser.host.render_next(), host_web::RESULT_OK);
            let b = bits(browser.host.output_pcm().expect("browser output"));
            assert_eq!(b, c_abi.render(block), "step {step}: block {block} alike");
            block += 1;
            while browser.host.poll_meters() != 0 {}
            if let Some(records) = c_abi.meter_batches().pop() {
                let [record] = records.as_slice() else {
                    panic!("one master record: {records:?}")
                };
                readings = Some(*record);
            }
        }
        let c_record = readings.expect("a C ABI batch");
        assert_eq!(
            (c_record.handle, c_record.component),
            (1, protocol::MeterComponent::Aggregate)
        );
        let c_reading = (c_record.flags == 1).then_some(c_record.value.to_bits());
        if c_record.flags != 1 {
            assert_eq!(
                c_record.value.to_bits(),
                0,
                "step {step}: an absent record is 0.0"
            );
        }
        let frame = browser.host.meter_frame();
        let strips = (frame.len() - 3) / 3;
        let b_reading = (browser.host.meter_header().master_gr_present == 1)
            .then(|| frame[3 * strips + 2].to_bits());
        assert_eq!(
            b_reading, c_reading,
            "step {step}: the master's reading alike"
        );
        match step {
            1 => assert!(
                c_reading.is_some_and(|db| f32::from_bits(db) > 0.0),
                "step 1: the compressor reduces: {c_reading:?}"
            ),
            _ => assert_eq!(c_reading, None, "step {step}: no master reading"),
        }
    }
}

/// The blocks #1690b's parity row plays per step: a whole number of the browser's meter windows
/// (12 blocks) and of the C ABI's at 44.1 kHz (11 blocks), and of the tap's window.
const MASTER_PLAY_BLOCKS: u64 = 132;

/// #1690b gate 1's set-up after the rebuild: both hosts arm `t1`'s compressor tap with a
/// four-block window, and the C ABI configures `[{master, gain reduction}]` with one batch per
/// meter window; the browser takes its meter lease.
fn arm_master_reading(browser: &mut Rig, c_abi: &mut CAbi) {
    // Both hosts arm by the same `Observe` transaction.
    let arm = [SessionEdit::Observe {
        strip_id: id("t1"),
        section: controls::TapSection::Inserts(id(COMPRESSOR.id)),
        tap_id: 1,
        armed: true,
        window_blocks: 4,
        arm_tag: 0,
    }];
    let (b, _) = browser.transaction_reply(&arm);
    assert!(
        matches!(b, Verdict::Applied { .. }),
        "the browser arm: {b:?}"
    );
    let (c, _) = c_abi.transaction(&arm);
    assert!(matches!(c, Verdict::Applied { .. }), "the C ABI arm: {c:?}");
    let window = host_core::default_meter_window_blocks(c_abi.rate, QUANTUM).get();
    assert_eq!(
        MASTER_PLAY_BLOCKS % u64::from(window),
        0,
        "whole C ABI windows"
    );
    let telemetry = protocol::TelemetryConfiguration {
        meters: vec![protocol::MeterEntry {
            target: protocol::MeterTarget::Master,
            metric: protocol::MeterMetric::GainReduction,
        }],
        meter_period_blocks: window,
        meter_window_blocks: None,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: false,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
    };
    let revision = c_abi.revision;
    c_abi.send(
        CommandPayload::TelemetryConfigure(&telemetry),
        ExpectedRevision::Exact(SessionRevision(revision)),
    );
    assert_eq!(browser.host.set_meter_lease(true), host_web::RESULT_OK);
}

impl CAbi {
    /// Every `METER_BATCH` on the lossy lane, in order, as its records; other lossy events are
    /// dropped.
    fn meter_batches(&mut self) -> Vec<Vec<protocol::MeterRecord>> {
        let mut batches = Vec::new();
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
                capi::miso_engine_v1_dequeue_event(self.session, EVENT_LANE_LOSSY, &mut out)
            };
            assert_eq!(code, C_RESULT_OK);
            if out.required_bytes == 0 {
                return batches;
            }
            let mut fields = [0_u16; 64];
            if let protocol::DecodedTypedEventFrame {
                payload: protocol::DecodedEventPayload::MeterBatch(batch),
                ..
            } = ProtocolCodec::default()
                .decode_typed_event(
                    &event[..out.required_bytes as usize],
                    &mut DecodeScratch::new(&mut fields),
                )
                .expect("lossy event")
            {
                batches.push(
                    (0..usize::from(batch.count))
                        .map(|index| batch.record(index).expect("meter record"))
                        .collect(),
                );
            }
        }
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
#[cfg(feature = "test-support")]
fn c_latency(c_abi: &CAbi) -> u64 {
    // SAFETY: Every field of the report is an integer, for which zero is a valid value.
    let mut report: capi::PlanResourceReport = unsafe { core::mem::zeroed() };
    report.struct_size = capi::PLAN_RESOURCE_REPORT_SIZE;
    // SAFETY: The plan is live; `report` is one complete local report.
    let code = unsafe { capi::miso_engine_v1_plan_resources(c_abi.plan, &mut report) };
    assert_eq!(code, C_RESULT_OK);
    report.latency_samples
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
        // The removed strip ramps out, then the rebuild is adopted at `S` (#1325 D3).
        let mut block = 4;
        while c_abi.watermark().0 < c_abi.revision {
            assert!(block < 64, "{rate} Hz: the rebuild is adopted");
            browser.step(block);
            c_abi.step(block);
            block += 1;
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
        // The limiters restart T1: a duck-swap, adopted at its `S` (#1324a).
        let revision = c_abi.watermark().0 + 1;
        step_both_until_applied(&mut browser, &mut c_abi, 4, revision);
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

/// One refused source call on both hosts: its row label, the expected `(result, diagnostic)`, the
/// browser call and the C ABI call. Each call returns its result code.
type RefusalRow<'a> = (
    &'a str,
    (u32, &'a [u8]),
    Box<dyn Fn(&mut Rig) -> u32 + 'a>,
    Box<dyn Fn(*mut Session) -> u32 + 'a>,
);

/// #1706 gate 1: every source call a host can refuse (submit, seek, anchored seek, session seek,
/// seek report) reports the same result code and the same diagnostic on both hosts, read through
/// the browser's session diagnostic and the C ABI's `miso_engine_v1_last_error`. The rows run in
/// one order on both rigs, and no two neighbours share a text, so a host that leaves the previous
/// diagnostic in place reads red.
///
/// Test value: red when either host maps a refusal kind to another code, drops or rewords the
/// core's text (the browser wrote none before #1706), or refuses a malformed or unknown ID before
/// the core with a code or text of its own (the browser's own unknown-ID lookup in `seek_source`),
/// when the browser's control response (what the page reads) does not carry the text, or when a
/// later successful source call leaves the refusal's text in it.
#[test]
fn a_refused_source_call_reports_the_same_result_and_diagnostic_on_both_hosts() {
    const RATE: u32 = 48_000;
    let model = model_at(RATE);
    let mut browser = Rig::new(&model);
    let c_abi = CAbi::new(&model);
    let [left, right] = planes(0, 0);
    let pointers = [left.as_ptr(), right.as_ptr()];
    let invalid = host_web::RESULT_INVALID_ARGUMENT;
    let submit = |rig: &mut Rig, id: &[u8], rate: u32| {
        rig.host
            .submit_source(id, 1, 0, rate, &[&left, &right], QUANTUM, false)
    };
    let c_submit = |session: *mut Session, id: &[u8], rate: u32| {
        let chunk = SourceChunk {
            struct_size: SOURCE_CHUNK_SIZE,
            sample_rate_hz: rate,
            generation: 1,
            start_frame: 0,
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
        // SAFETY: The session is live and used by this thread alone; the ID, the chunk's planes
        // and the report are owned storage that outlives the synchronous call.
        unsafe {
            capi::miso_engine_v1_source_submit_planar_f32(
                session,
                id.as_ptr(),
                id.len() as u64,
                &chunk,
                &mut report,
            )
        }
    };
    let c_seek = |session: *mut Session, id: &[u8], generation: u64| {
        // SAFETY: For every C ABI row below, the session is live and used by this thread alone, and
        // the ID and any report are owned storage that outlives the synchronous call.
        unsafe {
            capi::miso_engine_v1_source_seek(session, id.as_ptr(), id.len() as u64, generation, 0)
        }
    };
    let c_seek_at = |session: *mut Session, id: &[u8], generation: u64, anchor: u64| {
        // SAFETY: As above.
        unsafe {
            capi::miso_engine_v1_source_seek_at(
                session,
                id.as_ptr(),
                id.len() as u64,
                generation,
                0,
                anchor,
            )
        }
    };
    let c_seek_report = |session: *mut Session, id: &[u8]| {
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
        // SAFETY: As above.
        unsafe {
            capi::miso_engine_v1_source_seek_report(session, id.as_ptr(), id.len() as u64, &mut out)
        }
    };
    let unknown: &[u8] = b"src-absent";
    let known: &[u8] = b"src-t1";
    let rows: Vec<RefusalRow<'_>> = vec![
        (
            "seek, unknown ID",
            (invalid, b"source.id.unknown"),
            Box::new(|rig| rig.host.seek_source(unknown, 2, 0)),
            Box::new(|session| c_seek(session, unknown, 2)),
        ),
        (
            "seek, empty ID",
            (invalid, b"source.id.invalid"),
            Box::new(|rig| rig.host.seek_source(b"", 2, 0)),
            Box::new(|session| c_seek(session, b"", 2)),
        ),
        (
            "seek report, undeclared source",
            (invalid, b"source.id.unknown"),
            Box::new(|rig| rig.host.read_seek_report(unknown)),
            Box::new(|session| c_seek_report(session, unknown)),
        ),
        (
            "seek report, non-UTF-8 ID",
            (invalid, b"source.id.invalid"),
            Box::new(|rig| rig.host.read_seek_report(b"\xff")),
            Box::new(|session| c_seek_report(session, b"\xff")),
        ),
        (
            "submit, unknown ID",
            (invalid, b"source.id.unknown"),
            Box::new(|rig| submit(rig, unknown, RATE)),
            Box::new(|session| c_submit(session, unknown, RATE)),
        ),
        (
            "seek_at, empty ID",
            (invalid, b"source.id.invalid"),
            Box::new(|rig| rig.host.seek_source_at(b"", 2, 0, 0)),
            Box::new(|session| c_seek_at(session, b"", 2, 0)),
        ),
        (
            "seek, stale generation",
            (invalid, b"source.generation.stale"),
            Box::new(|rig| rig.host.seek_source(known, 1, 0)),
            Box::new(|session| c_seek(session, known, 1)),
        ),
        (
            "seek_at, unaligned anchor",
            (invalid, b"source.seek.anchor_unaligned"),
            Box::new(|rig| rig.host.seek_source_at(known, 2, 0, 1)),
            Box::new(|session| c_seek_at(session, known, 2, 1)),
        ),
        (
            "session seek, stale generation",
            (invalid, b"source.generation.stale"),
            Box::new(|rig| rig.host.session_seek(1, 0, None)),
            // SAFETY: As above.
            Box::new(|session| unsafe { capi::miso_engine_v1_session_seek(session, 1, 0) }),
        ),
        (
            "session seek_at, unaligned anchor",
            (invalid, b"source.seek.anchor_unaligned"),
            Box::new(|rig| rig.host.session_seek(2, 0, Some(1))),
            // SAFETY: As above.
            Box::new(|session| unsafe { capi::miso_engine_v1_session_seek_at(session, 2, 0, 1) }),
        ),
        (
            "submit, wrong sample rate",
            (invalid, b"source.rate.mismatch"),
            Box::new(|rig| submit(rig, known, RATE / 2)),
            Box::new(|session| c_submit(session, known, RATE / 2)),
        ),
    ];
    for (label, (code, text), on_browser, on_c_abi) in &rows {
        let browser_code = on_browser(&mut browser);
        let browser_reply = (browser_code, browser.host.diagnostic().to_vec());
        let c_code = on_c_abi(c_abi.session);
        let c_reply = (c_code, c_abi.last_error());
        assert_eq!(browser_reply, c_reply, "{label}: the two hosts");
        assert_eq!(
            c_reply,
            (*code, text.to_vec()),
            "{label}: the core's refusal"
        );
        // The page reads the text through `miso_engine_web_v1_control_response_ptr/_bytes`.
        assert_eq!(
            browser.host.control_response(),
            *text,
            "{label}: the browser's control response"
        );
    }
    // A source call that succeeds after a refusal leaves no stale text in the response.
    assert_eq!(
        browser.host.read_seek_report(known),
        host_web::RESULT_OK,
        "the seek report of a declared source"
    );
    assert!(
        browser.host.control_response().is_empty(),
        "a successful source call clears the response"
    );
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

/// #2205 parity row: the at-once session seek, the C ABI's `miso_engine_v1_session_seek_at_once`
/// and the browser's `session_prepare_seek`, lands at the next block on both hosts: equal seek
/// reports naming that block, and equal output bits.
///
/// Test value: red if one host's at-once form waits for a runway (its report keeps generation 1
/// at the block) or lands at another block than the other's.
#[cfg(feature = "test-support")]
#[test]
fn the_at_once_session_seek_lands_alike_on_both_hosts() {
    let quantum = u64::from(QUANTUM);
    let model = model_at(48_000);
    let mut browser = Rig::new(&model);
    let mut c_abi = CAbi::new(&model);
    for block in 0..4 {
        session_block(
            &mut browser,
            &mut c_abi,
            block,
            [Some((1, block)); 3],
            "boot",
        );
    }
    let frame = 40 * quantum;
    assert_eq!(
        browser.host.session_prepare_seek(2, frame),
        host_web::RESULT_OK
    );
    // SAFETY: The session is live and used by this thread alone.
    let code = unsafe { capi::miso_engine_v1_session_seek_at_once(c_abi.session, 2, frame) };
    assert_eq!(code, C_RESULT_OK, "C ABI at-once session seek");
    session_block(&mut browser, &mut c_abi, 4, [Some((2, 40)); 3], "at once");
    let reports = same_session_reports(&mut browser, &c_abi, "at once");
    assert!(
        reports
            .iter()
            .all(|report| report.0 == 2 && report.1 == 4 * quantum),
        "every source lands at the next block on both hosts: {reports:?}"
    );
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
            browser.host.session_seek(2, frame, None),
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
            browser.host.session_seek(3, late_frame, Some(anchor)),
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
            browser.host.session_seek(2, 0, Some(anchor)),
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
            browser.host.session_seek(2, 0, None),
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
        assert_eq!(browser.host.session_seek(3, 0, None), host_web::RESULT_OK);
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
/// made only the last, on both hosts (render reads the newest slot of the offsets cell, whole);
/// after 28 blocks a second burst of ten (ending at -2 dB) again gives the twin's bits when the
/// twin makes only its last, and after each burst's jump the output equals the committed
/// session's boot.
/// Three nested VCAs reach `t2`, declared and nested out of ID order, at 24, -24 and 2e-30 dB in
/// ascending ID order: a move of the third to 1e-30 is `live` and renders the committed session's
/// boot (the `S` bits it writes are the classifier gate's,
/// `a_vca_move_writes_the_automated_lanes_offsets_cell`).
///
/// Test value: red if render applies an older slot than the newest committed one, if render hears
/// only the first write of a cell (the second burst would not be heard), or if a nested reach is
/// composed otherwise than preparation.
#[test]
fn many_vca_moves_before_one_render_are_the_last_alike_on_both_hosts() {
    const EDIT: u64 = 4;
    // Each burst's run: its jump settles inside it at every rate, and two fit the sources.
    const SPAN: u64 = 28;
    for rate in RATES {
        let model = automated_vca_model(rate, |_| {});
        let mut many = Rig::new(&model);
        let mut one = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        same_blocks(&mut many, &mut c_abi, 0..EDIT, "before the moves");
        for block in 0..EDIT {
            one.step(block);
        }
        for (burst, last) in [(0_u64, -6.0_f32), (1, -2.0)] {
            for step in 1..=10 {
                let db = if step == 10 {
                    last
                } else {
                    -(step as f32) - 0.5 * burst as f32
                };
                let edit = [vca_offset(VCA.id, db, None)];
                assert_eq!(many.transaction(&edit), c_abi.transaction(&edit).0);
            }
            one.transaction(&[vca_offset(VCA.id, last, None)]);
            // The committed session's boot: a twin that also missed every write after the first
            // would still equal `many`, so each burst is held to the boot after its jump too.
            let committed = snapshot_bytes(|payload, expected| many.command(payload, expected));
            let mut boot = Rig::boot(&committed, rate);
            let first = EDIT + SPAN * burst;
            let settled = first + settle_blocks(&mut many, model.control_smoothing_samples().fader);
            assert!(
                settled < first + SPAN,
                "{rate} Hz: the jump settles inside the burst's run"
            );
            for block in 0..first {
                boot.step(block);
            }
            for block in first..first + SPAN {
                let (m, c, o) = (many.step(block), c_abi.step(block), one.step(block));
                let b = boot.step(block);
                assert_eq!(m, c, "{rate} Hz: block {block} alike on both hosts");
                assert_eq!(m, o, "{rate} Hz: block {block} is the last move's");
                if block >= settled {
                    assert_eq!(m, b, "{rate} Hz: block {block} renders the committed boot");
                }
            }
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
            headers_only: false,
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
        headers_only: false,
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

/// #1652 gate 1's session on both hosts: the command session at 48 kHz with no console slot and
/// no insert (so no strip state outlives a mute), whose `t1` sends `pre_fader` with
/// `follows_mute` into bus `b`; `t1` gets a mute entry on `both` with steps 0, 1, 0 at 1,000 and
/// 2,500 (`entry`), or is muted statically (`muted`). `control_smoothing` is explicit: the mute
/// length is 480 samples.
fn follow_model(entry: bool, muted: bool) -> SessionModel {
    let mut model = model_at(48_000);
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
    }
    for submix in &mut model.submixes {
        submix.console.clear();
        submix.inserts.effects.clear();
    }
    model.control_smoothing = Some(session::ControlSmoothing {
        mute_ms: 10.0,
        fader_ms: 5.0,
        pan_ms: 5.0,
    });
    let send = model
        .routes
        .iter()
        .find(|route| route.id.as_str() == "t1-b")
        .expect("the command session's following send");
    assert!(send.follows_mute);
    let t1 = model
        .tracks
        .iter_mut()
        .find(|track| track.id.as_str() == T1.id)
        .expect("t1");
    (t1.fader.left_mute, t1.fader.right_mute) = (muted, muted);
    if entry {
        let step = |start: u64, end: u64, from: f32, to: f32| session::AutomationSegment {
            shape: session::AutomationShape::Step,
            start_sample: start,
            end_sample: end,
            start_value: from,
            end_value: to,
            unit: session::ParameterUnit::Linear,
        };
        model.automation.push(session::Automation {
            id: id("t1-mute"),
            target: address(
                Owner::Strip(id(T1.id)),
                Section::Builtins,
                MUTE,
                ParameterChannel::Both,
            ),
            segments: vec![step(0, 1_000, 0.0, 1.0), step(1_000, 2_500, 1.0, 0.0)],
            offset: session::AutomationOffset::default(),
        });
    }
    model
}

/// #1652 U2a fold 2 (MAJOR-A, conservative; #1713 deletes the flag) on both hosts. A rebuild
/// whose send join carries a following route into a six-word one (it removes `t1`'s mute entry
/// while `t1-b` follows it, sent at block 8 while the send's columns ramp toward the mute the
/// curve stepped on at 1,000) completes as the transition fallback; a rebuild that keeps the
/// entry (the unmute moved to 3,000: `t1-b` stays following) and a rebuild of a session with no
/// following route (a fader ride added to `t1`, no move in flight) complete exactly. Both hosts
/// take the one decision in the shared control plane: the browser's status words carry the C
/// ABI's watermark, flags included, and both render the same bits.
///
/// Test value: red if the shared control plane does not mark a seven -> six rebuild, marks a
/// rebuild whose following route stays following or one with no following route, or if either
/// host decides the outcome by a rule of its own.
#[test]
fn a_rebuild_that_drops_a_following_route_to_six_words_reports_the_fallback_alike() {
    let step = |start: u64, end: u64, from: f32, to: f32| session::AutomationSegment {
        shape: session::AutomationShape::Step,
        start_sample: start,
        end_sample: end,
        start_value: from,
        end_value: to,
        unit: session::ParameterUnit::Linear,
    };
    let later_unmute = SessionEdit::UpsertAutomation {
        automation: session::Automation {
            id: id("t1-mute"),
            target: address(
                Owner::Strip(id(T1.id)),
                Section::Builtins,
                MUTE,
                ParameterChannel::Both,
            ),
            segments: vec![step(0, 1_000, 0.0, 1.0), step(1_000, 3_000, 1.0, 0.0)],
            offset: session::AutomationOffset::default(),
        },
    };
    let ride = SessionEdit::UpsertAutomation {
        automation: session::Automation {
            id: id("t1-fader-ride"),
            target: address(
                Owner::Strip(id(T1.id)),
                Section::Builtins,
                5,
                ParameterChannel::Both,
            ),
            segments: vec![session::AutomationSegment {
                shape: session::AutomationShape::Linear,
                start_sample: 0,
                end_sample: 1_000_000,
                start_value: 0.0,
                end_value: -12.0,
                unit: session::ParameterUnit::Db,
            }],
            offset: session::AutomationOffset::default(),
        },
    };
    let remove = SessionEdit::RemoveAutomation {
        automation_id: id("t1-mute"),
    };
    for (label, entry, edit, expected) in [
        (
            "seven -> six, a column mid-move",
            true,
            remove,
            capi::OUTCOME_TRANSITION_FALLBACK,
        ),
        ("seven -> seven", true, later_unmute, capi::OUTCOME_EXACT),
        ("no following route", false, ride, capi::OUTCOME_EXACT),
    ] {
        let model = follow_model(entry, false);
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        same_blocks(&mut browser, &mut c_abi, 0..8, label);
        let (b, b_reply) = browser.transaction_reply(std::slice::from_ref(&edit));
        let (c, c_reply) = c_abi.transaction(std::slice::from_ref(&edit));
        assert_eq!(b, c, "{label}: one verdict");
        assert_eq!(b_reply, c_reply, "{label}: one reply");
        assert_eq!(
            b,
            Verdict::Applied {
                path: EditPath::Rebuild,
                revision: model.revision + 1
            },
            "{label}: the edit rebuilds"
        );
        same_blocks(&mut browser, &mut c_abi, 8..12, label);
        let watermark = c_abi.watermark();
        assert_eq!(
            status_watermark(&browser),
            watermark,
            "{label}: the browser status carries the C ABI's watermark"
        );
        assert_eq!(
            (watermark.0, watermark.2),
            (model.revision + 1, expected),
            "{label}: the rebuild's revision and outcome flags"
        );
    }
}

/// #1653: one `SET_CONTROL` of `value` on `owner`'s own row `control` (a route or a VCA row, or a
/// strip's solo).
fn own_edit(owner: Owner, control: u32, value: f32) -> SessionEdit {
    SessionEdit::set_control(
        address(owner, Section::Own, control, Lane::Both),
        value,
        None,
    )
}

/// #1653: `edits` on both hosts as one transaction each: one verdict, one reply, path `path`.
fn both_commit(
    browser: &mut Rig,
    c_abi: &mut CAbi,
    edits: &[SessionEdit],
    path: EditPath,
    label: &str,
) {
    let (b, b_reply) = browser.transaction_reply(edits);
    let (c, c_reply) = c_abi.transaction(edits);
    assert_eq!(b, c, "{label}: one verdict");
    assert_eq!(b_reply, c_reply, "{label}: one reply");
    assert!(
        matches!(b, Verdict::Applied { path: applied, .. } if applied == path),
        "{label}: {b:?}, not {path:?}"
    );
}

/// #1653: `model` on both hosts for `blocks` blocks, with `edits[i] = (block, edits, path)` sent
/// before block `block` renders. Both hosts render the same bits each block; returns them.
fn both_run(
    model: &SessionModel,
    blocks: u64,
    edits: &[(u64, Vec<SessionEdit>, EditPath)],
) -> Vec<Vec<u32>> {
    let mut browser = Rig::new(model);
    let mut c_abi = CAbi::new(model);
    let mut out = Vec::new();
    for block in 0..blocks {
        for (at, edit, path) in edits {
            if *at == block {
                both_commit(
                    &mut browser,
                    &mut c_abi,
                    edit,
                    *path,
                    &format!("block {block}"),
                );
            }
        }
        let (b, c) = (browser.step(block), c_abi.step(block));
        assert_eq!(b, c, "block {block}: the two hosts agree");
        out.push(b);
    }
    out
}

/// #1653 gate 1 on both hosts. `t1` is automated-muted from 1,000 (480-sample mute ramps) and
/// `t1-b` follows it. Before block 8 (1,024) a `SET_CONTROL` turns `t1-b`'s `follows_mute` off,
/// before block 15 (1,920) one turns it on: on both hosts each is `live`, with one reply. From the
/// first block boundary after each 480-sample route ramp, each host's output is bit for bit a
/// fresh plan of the committed model (fed the same PCM from 0): the send open while the curve
/// mutes, then shut until the curve's unmute. Not vacuous: the two committed models' plans differ
/// in the first window.
///
/// Test value: red if the browser or the C ABI loses a live `follows_mute` toggle on an automated
/// route (the commit reads `follows_mute` from the prepared route only, or refuses the seven-word
/// record), or if the hosts disagree.
#[test]
fn a_follows_mute_toggle_on_a_following_route_is_live_on_both_hosts() {
    const BLOCKS: u64 = 32;
    let quantum = u64::from(QUANTUM);
    let toggle = |on: bool| {
        vec![own_edit(
            Owner::Route(id("t1-b")),
            ROUTE_FOLLOWS_MUTE_ROW,
            f32::from(u8::from(on)),
        )]
    };
    let following = follow_model(true, false);
    let live = both_run(
        &following,
        BLOCKS,
        &[
            (8, toggle(false), EditPath::Live),
            (15, toggle(true), EditPath::Live),
        ],
    );
    let mut open = following.clone();
    for route in &mut open.routes {
        if route.id.as_str() == "t1-b" {
            route.follows_mute = false;
        }
    }
    let fresh_open = both_run(&open, 15, &[]);
    let fresh_shut = both_run(&following, BLOCKS, &[]);
    let first_after = |sample: u64| sample.div_ceil(quantum);
    let off = first_after(8 * quantum + 480)..15;
    for block in off.clone() {
        assert_eq!(
            live[block as usize], fresh_open[block as usize],
            "block {block}: the plan with follows_mute off"
        );
    }
    for block in first_after(15 * quantum + 480)..BLOCKS {
        assert_eq!(
            live[block as usize], fresh_shut[block as usize],
            "block {block}: the plan with follows_mute on"
        );
    }
    assert!(
        off.clone()
            .any(|block| fresh_open[block as usize] != fresh_shut[block as usize]),
        "the toggle is audible"
    );
}

/// #1653 gate 2 on both hosts: solo and a VCA mute compose with `t1`'s mute curve for `t1-b` as
/// for the strip. `t1` has its mute entry (curve 1 on `[1,000, 2,500)`) and, for the VCA case, is
/// VCA `v`'s only member and sends `t1-pre` from its `pre_fader` tap straight to the output.
///
/// - Solo `t2` before block 2 (the curve is 0): from the first block boundary after the 480-sample
///   ramps to block 14, both hosts render what the session with no mute entry renders under the
///   same solo: `t1` muted by solo, its send shut with it. Unsolo before block 14 (the curve is 1):
///   from block 20 on, the curve's own plan with no solo (the send stays shut by the curve, and
///   reopens with it at 2,500).
/// - A VCA mute of `v` before block 2 and its release before block 14: the same comparisons.
/// - A route into the output never follows: under the VCA mute `t1-pre` still plays (the session
///   without it renders other bits), while the twin comparisons hold.
///
/// Test value: red if solo or a VCA mute overwrites the curve instead of composing with it for the
/// send (the send reopens on the release while the curve still mutes, or does not follow the solo
/// while the curve is 0), or if the hosts disagree.
#[test]
fn solo_and_a_vca_mute_compose_with_the_curve_for_a_following_send_on_both_hosts() {
    const BLOCKS: u64 = 32;
    let solo = |on: bool| {
        vec![own_edit(
            Owner::Strip(id(T2.id)),
            SOLO,
            f32::from(u8::from(on)),
        )]
    };
    let vca_mute = |on: bool| vec![own_edit(Owner::Vca(id("v")), 2, f32::from(u8::from(on)))];
    // `t1` alone in `v`, with a pre-fader route straight to the output (`pre_out`).
    let with_vca = |mut model: SessionModel, pre_out: bool| {
        for vca in &mut model.vcas {
            if vca.id.as_str() == "v" {
                vca.members = vec![id(T1.id)];
            }
        }
        if pre_out {
            let mut route = model
                .routes
                .iter()
                .find(|route| route.id.as_str() == "t1-b")
                .expect("the send")
                .clone();
            route.id = id("t1-pre");
            route.follows_mute = false;
            route.destination = model
                .routes
                .iter()
                .find(|route| {
                    matches!(
                        route.destination,
                        session::RouteDestination::OutputInput { .. }
                    )
                })
                .expect("a route to the output")
                .destination
                .clone();
            model.routes.push(route);
            model.routes.sort_by(|left, right| left.id.cmp(&right.id));
        }
        model
    };
    for (label, edit, path, curve, twin, bare) in [
        (
            "solo",
            &solo as &dyn Fn(bool) -> Vec<SessionEdit>,
            EditPath::Monitor,
            follow_model(true, false),
            follow_model(false, false),
            None,
        ),
        (
            "vca",
            &vca_mute as &dyn Fn(bool) -> Vec<SessionEdit>,
            EditPath::Live,
            with_vca(follow_model(true, false), true),
            with_vca(follow_model(false, false), true),
            Some(with_vca(follow_model(true, false), false)),
        ),
    ] {
        let composed = both_run(
            &curve,
            BLOCKS,
            &[(2, edit(true), path), (14, edit(false), path)],
        );
        let static_twin = both_run(&twin, 14, &[(2, edit(true), path)]);
        let curve_only = both_run(&curve, BLOCKS, &[]);
        for block in 6..14 {
            assert_eq!(
                composed[block], static_twin[block],
                "{label}, block {block}: muted by the term while the curve is 0, then by both"
            );
        }
        assert_ne!(
            composed[6], curve_only[6],
            "{label}: the term mutes t1 and its send while the curve is 0"
        );
        for block in 20..BLOCKS as usize {
            assert_eq!(
                composed[block], curve_only[block],
                "{label}, block {block}: the release restores the curve's state"
            );
        }
        if let Some(bare) = bare {
            let without = both_run(&bare, 14, &[(2, edit(true), path)]);
            assert!(
                (6..14).any(|block| without[block] != composed[block]),
                "{label}: the route into the output plays while t1 is muted"
            );
        }
    }
}

/// An entry `name` on `t1-b`'s own row `row` (1 `gain_db`, 2 `mute`, 3-6 the matrix), one segment
/// of `shape` over `[0, end)` from `from` to `to` in `unit`.
#[cfg(feature = "test-support")]
fn send_row_entry(
    name: &str,
    row: u32,
    shape: session::AutomationShape,
    (from, to): (f32, f32),
    unit: session::ParameterUnit,
) -> session::Automation {
    session::Automation {
        id: id(name),
        target: address(
            Owner::Route(id("t1-b")),
            Section::Own,
            row,
            ParameterChannel::Both,
        ),
        segments: vec![session::AutomationSegment {
            shape,
            start_sample: 0,
            end_sample: u64::from(QUANTUM) * BLOCKS,
            start_value: from,
            end_value: to,
            unit,
        }],
        offset: session::AutomationOffset::default(),
    }
}

/// #1654 gate 5 on both hosts: an automation entry on the route into the output `t1-main` is
/// refused `control.automation.target` alike, with one diagnostic.
///
/// Test value: red if a route into the output's rows became automatable on either host (D8).
#[cfg(feature = "test-support")]
#[test]
fn an_output_route_entry_is_refused_alike_on_both_hosts() {
    let model = follow_model(false, false);
    let mut browser = Rig::new(&model);
    let mut c_abi = CAbi::new(&model);
    same_blocks(&mut browser, &mut c_abi, 0..1, "before");
    let mut entry = send_row_entry(
        "t1-main-gain",
        1,
        session::AutomationShape::Linear,
        (-6.0, -6.0),
        session::ParameterUnit::Db,
    );
    entry.target.owner = Owner::Route(id("t1-main"));
    let edits = [SessionEdit::UpsertAutomation { automation: entry }];
    let frame = browser.encode(
        CommandPayload::SessionTransactionApply(&edits),
        ExpectedRevision::Exact(SessionRevision(browser.revision)),
    );
    let (b_code, b_diagnostic) = browser.send_frame(&frame);
    let (c_code, c_diagnostic) = c_abi.send_status(
        CommandPayload::SessionTransactionApply(&edits),
        ExpectedRevision::Exact(SessionRevision(c_abi.revision)),
    );
    // Each host has its own result numbering; the diagnostic is the shared contract.
    assert_ne!(b_code, C_RESULT_OK, "the browser refuses the entry");
    assert_ne!(c_code, C_RESULT_OK, "the C ABI refuses the entry");
    assert_eq!(b_diagnostic, c_diagnostic, "one diagnostic");
    let diagnostic = String::from_utf8_lossy(&c_diagnostic);
    assert!(
        diagnostic.contains("control.automation.target")
            && diagnostic.contains("$.automation[id=t1-main-gain].target"),
        "{diagnostic}"
    );
    same_blocks(&mut browser, &mut c_abi, 1..3, "after the refusal");
}

/// Root ruling B19-R10 (send-row rendering is not shipped; #1654 turns it on behind #1718). A
/// session whose send `t1-b` stores automation on its own rows (a `gain_db` ride, a `mute` step,
/// rides and flat entries on `ll`, `lr`, `rl` and `rr`) behaves exactly as the session without
/// those entries, as on `main` before batch 19: on both hosts, with `t1-b` six-word (no source
/// mute curve) and seven-word (`t1`'s stored mute), the session prepares with no refusal, every
/// block renders the bits of the session without the entries (through a live `gain_db` move on
/// the send and an inert entry upsert, which commits `model_only`), and the control list and
/// state pages are those of the session without the entries (no send row lists `automated`).
///
/// Test value: red if a send-row entry reaches render (a cell, a seven-word lane or a hand
/// offset: the bits move), if it is refused or changes an edit's path, or if the control list
/// flags or values a send row from it.
#[cfg(feature = "test-support")]
#[test]
fn stored_send_row_automation_is_inert_alike_on_both_hosts() {
    use session::{AutomationShape, ParameterUnit};
    const RUN: u64 = 32;
    let gain = send_row_edit(1, -6.0, Some(480));
    let upsert = vec![SessionEdit::UpsertAutomation {
        automation: send_row_entry(
            "t1-b-lr",
            4,
            AutomationShape::Linear,
            (0.7, 0.7),
            ParameterUnit::Linear,
        ),
    }];
    let edits = [(4, gain, EditPath::Live), (10, upsert, EditPath::ModelOnly)];
    for (label, base) in [
        ("six-word send", follow_model(false, false)),
        ("seven-word send", follow_model(true, false)),
    ] {
        let mut rows = base.clone();
        rows.automation.extend([
            send_row_entry(
                "t1-b-gain",
                1,
                AutomationShape::Linear,
                (0.0, -12.0),
                ParameterUnit::Db,
            ),
            send_row_entry(
                "t1-b-mute",
                2,
                AutomationShape::Step,
                (0.0, 1.0),
                ParameterUnit::Linear,
            ),
            send_row_entry(
                "t1-b-ll",
                3,
                AutomationShape::Linear,
                (0.1, 0.9),
                ParameterUnit::Linear,
            ),
            send_row_entry(
                "t1-b-lr",
                4,
                AutomationShape::Linear,
                (0.5, 0.5),
                ParameterUnit::Linear,
            ),
            send_row_entry(
                "t1-b-rl",
                5,
                AutomationShape::Linear,
                (0.25, 0.25),
                ParameterUnit::Linear,
            ),
            send_row_entry(
                "t1-b-rr",
                6,
                AutomationShape::Linear,
                (0.9, 0.1),
                ParameterUnit::Linear,
            ),
        ]);
        let (mut browser, mut c_abi) = (Rig::new(&rows), CAbi::new(&rows));
        let (mut plain_browser, mut plain_c_abi) = (Rig::new(&base), CAbi::new(&base));
        let (b_rows, _) = dump(|payload, expected| browser.command(payload, expected));
        let (c_rows, _) = dump(|payload, expected| c_abi.send(payload, expected));
        let (plain_rows, _) = dump(|payload, expected| plain_browser.command(payload, expected));
        let (plain_c_rows, _) = dump(|payload, expected| plain_c_abi.send(payload, expected));
        assert_eq!(b_rows, c_rows, "{label}: the hosts list alike");
        assert_eq!(
            plain_rows, plain_c_rows,
            "{label}: the hosts list alike, no entries"
        );
        assert_eq!(
            b_rows, plain_rows,
            "{label}: the list ignores the send-row entries"
        );
        assert!(
            !b_rows
                .iter()
                .any(|row| row.contains("\"t1-b\"") && row.contains("automated: true")),
            "{label}: no send row lists automated"
        );
        let run = both_run(&rows, RUN, &edits);
        let plain = both_run(&base, RUN, &edits);
        for (block, (run, plain)) in run.iter().zip(&plain).enumerate() {
            assert_eq!(run, plain, "{label}, block {block}: the entries are inert");
        }
    }
}

/// `t1-b`'s own row `control` set to `value`, over `ramp` samples when given.
#[cfg(feature = "test-support")]
fn send_row_edit(control: u32, value: f32, ramp: Option<u32>) -> Vec<SessionEdit> {
    vec![SessionEdit::set_control(
        address(Owner::Route(id("t1-b")), Section::Own, control, Lane::Both),
        value,
        ramp,
    )]
}

/// [`follow_model`]`(false, false)` whose send `t1-b` follows a stored mute curve held open (`t1`'s
/// mute entry flat at 0 on both lanes): a seven-word following send whose curves never move, so a
/// live send move on it is the only ramp in flight.
#[cfg(feature = "test-support")]
fn open_follow_model() -> SessionModel {
    let mut model = follow_model(false, false);
    model.automation.push(session::Automation {
        id: id("t1-mute"),
        target: address(
            Owner::Strip(id(T1.id)),
            Section::Builtins,
            MUTE,
            ParameterChannel::Both,
        ),
        segments: vec![session::AutomationSegment {
            shape: session::AutomationShape::Step,
            start_sample: 0,
            end_sample: u64::from(QUANTUM) * BLOCKS,
            start_value: 0.0,
            end_value: 0.0,
            unit: session::ParameterUnit::Linear,
        }],
        offset: session::AutomationOffset::default(),
    });
    model
}

/// A rebuild that adds the muted, inactive send `t2-b` into the bus: no bit of the mix moves.
#[cfg(feature = "test-support")]
fn muted_send_rebuild(model: &SessionModel) -> Vec<SessionEdit> {
    let mut send = model
        .routes
        .iter()
        .find(|route| route.id.as_str() == "t1-b")
        .expect("the send")
        .clone();
    send.id = id("t2-b");
    send.source = session::RouteSource::Track {
        track_id: id(T2.id),
        tap: session::SendTap::PreFader,
    };
    send.mute = true;
    send.follows_mute = false;
    vec![SessionEdit::UpsertRoute { route: send }]
}

/// #1654 root ruling B19-R8 (MAJOR-1, MAJOR-2) and R19-5 on both hosts, on [`open_follow_model`].
/// MAJOR-1: a live send move (`gain_db` to -3 dB over 4,000 samples, from block 4) is in flight
/// when a rebuild before block 6 adds a muted, inactive send: the move keeps its own length (every
/// block is the never-rebuilt run's, bit for bit), and the rebuild completes
/// `TRANSITION_FALLBACK` (one watermark, count 1), the conservative term. The same rebuild before
/// block 44 (the move long finished) completes `EXACT`. MAJOR-2: one transaction before block 4
/// moves `gain_db` over 4,000 samples and adds the muted send: it completes `TRANSITION_FALLBACK`
/// from its own send move alone; the rebuild alone completes `EXACT`. Every block both hosts
/// render the same bits.
///
/// Test value: red if either host completes a rebuild that carries a following send `EXACT` while
/// a live send move (an earlier commit's or the transaction's own) may still ramp, if one is
/// flagged with no live send move, if the carry cuts the move, or if the hosts disagree.
#[cfg(feature = "test-support")]
#[test]
fn a_rebuild_carrying_a_following_send_across_a_live_send_move_is_flagged_alike_on_both_hosts() {
    const RUN: u64 = 48;
    const RAMP: u32 = 4_000;
    let model = open_follow_model();
    let unrelated = muted_send_rebuild(&model);
    let hand = send_row_edit(1, -3.0, Some(RAMP));
    let mut same_transaction = hand.clone();
    same_transaction.extend(unrelated.iter().cloned());
    let moved = both_run(&model, RUN, &[(4, hand.clone(), EditPath::Live)]);
    // (label, live move block, rebuild block, rebuild edits, flagged)
    for (label, live, rebuild, edits, flagged) in [
        ("MAJOR-1", Some(4), 6, &unrelated, true),
        (
            "MAJOR-1, the move long finished",
            Some(4),
            44,
            &unrelated,
            false,
        ),
        ("MAJOR-2", None, 4, &same_transaction, true),
        ("MAJOR-2, no move", None, 4, &unrelated, false),
    ] {
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        for block in 0..RUN {
            if live == Some(block) {
                both_commit(&mut browser, &mut c_abi, &hand, EditPath::Live, label);
            }
            if block == rebuild {
                both_commit(&mut browser, &mut c_abi, edits, EditPath::Rebuild, label);
            }
            let (b, c) = (browser.step(block), c_abi.step(block));
            assert_eq!(b, c, "{label}, block {block}: the two hosts agree");
            if live.is_some() {
                assert_eq!(
                    b, moved[block as usize],
                    "{label}, block {block}: the never-rebuilt run's bits"
                );
            }
            if block == rebuild {
                let watermark = c_abi.watermark();
                assert_eq!(browser.host.read_watermark(), host_web::RESULT_OK);
                assert_eq!(
                    status_watermark(&browser),
                    watermark,
                    "{label}: one watermark"
                );
                let outcome = if flagged {
                    capi::OUTCOME_TRANSITION_FALLBACK
                } else {
                    capi::OUTCOME_EXACT
                };
                assert_eq!(watermark.2, outcome, "{label}: the rebuild's outcome");
            }
        }
        assert_eq!(
            (
                browser.host.watermark().transition_fallback_count,
                c_abi.transition_fallback_count()
            ),
            (u64::from(flagged), u64::from(flagged)),
            "{label}: the fallback count"
        );
    }
}

/// #1654 root ruling B19-R9 (the B19-R8 check's repro) on both hosts, on [`open_follow_model`]
/// (480-sample landings at quantum 128). A live send gain move to -3 dB from 512 over 200 samples
/// is in flight when R1 (the direct route `t1-main` to -3 dB, block 5) rebuilds: it adopts at 640
/// flagged (count 1). R2 (`t1-main` to -6 dB, block 7, sample 896) comes after the move ended but
/// inside R1's at-once landing: `TRANSITION_FALLBACK` (count 2). R3 (`t1-main` to -9 dB, block
/// 16), after every landing ended with no live send move: `EXACT` (count stays 2). Every block
/// both hosts render the same bits.
///
/// Test value: red if either host's flagged rebuild does not fold its own at-once landing into the
/// send move window (R2 reads `EXACT`), if the window outlives that landing (R3 flagged), or if
/// the hosts disagree.
#[cfg(feature = "test-support")]
#[test]
fn a_second_rebuild_inside_a_flagged_rebuilds_landing_is_flagged_alike_on_both_hosts() {
    const RUN: u64 = 24;
    assert_eq!(QUANTUM, 128);
    let model = open_follow_model();
    let direct = |db: f32| {
        let mut route = model
            .routes
            .iter()
            .find(|route| route.id.as_str() == "t1-main")
            .expect("the direct route")
            .clone();
        route.gain_db = db;
        vec![SessionEdit::UpsertRoute { route }]
    };
    let hand = send_row_edit(1, -3.0, Some(200));
    // (label, rebuild block, edits, outcome, fallback count)
    let rebuilds = [
        ("R1", 5, direct(-3.0), capi::OUTCOME_TRANSITION_FALLBACK, 1),
        ("R2", 7, direct(-6.0), capi::OUTCOME_TRANSITION_FALLBACK, 2),
        ("R3", 16, direct(-9.0), capi::OUTCOME_EXACT, 2),
    ];
    let mut browser = Rig::new(&model);
    let mut c_abi = CAbi::new(&model);
    for block in 0..RUN {
        if block == 4 {
            both_commit(
                &mut browser,
                &mut c_abi,
                &hand,
                EditPath::Live,
                "the hand move",
            );
        }
        let rebuild = rebuilds.iter().find(|rebuild| rebuild.1 == block);
        if let Some((label, _, edits, _, _)) = rebuild {
            both_commit(&mut browser, &mut c_abi, edits, EditPath::Rebuild, label);
        }
        let (b, c) = (browser.step(block), c_abi.step(block));
        assert_eq!(b, c, "block {block}: the two hosts agree");
        if let Some((label, _, _, outcome, fallbacks)) = rebuild {
            let watermark = c_abi.watermark();
            assert_eq!(browser.host.read_watermark(), host_web::RESULT_OK);
            assert_eq!(
                status_watermark(&browser),
                watermark,
                "{label}: one watermark"
            );
            assert_eq!(
                watermark.1,
                block * u64::from(QUANTUM),
                "{label}: adopted at its submit block"
            );
            assert_eq!(watermark.2, *outcome, "{label}: the rebuild's outcome");
            assert_eq!(
                (
                    browser.host.watermark().transition_fallback_count,
                    c_abi.transition_fallback_count()
                ),
                (*fallbacks, *fallbacks),
                "{label}: the fallback count"
            );
        }
    }
}

// ---------------------------------------------------------------------------------------------
// #1655 V5: stored VCA offset and mute automation on both hosts
// ---------------------------------------------------------------------------------------------

/// #1655: one segment of `shape` over `[start, end)` from `from` to `to` in `unit`.
fn vca_piece(
    shape: session::AutomationShape,
    (start, end): (u64, u64),
    (from, to): (f32, f32),
    unit: session::ParameterUnit,
) -> session::AutomationSegment {
    session::AutomationSegment {
        shape,
        start_sample: start,
        end_sample: end,
        start_value: from,
        end_value: to,
        unit,
    }
}

/// #1655: an entry on VCA `vca`'s row `control` (1 `offset_db`, 2 `mute`) on both lanes, named
/// `{vca}-ride` or `{vca}-mute`.
fn vca_entry(
    vca: &str,
    control: u32,
    segments: Vec<session::AutomationSegment>,
) -> session::Automation {
    let suffix = if control == VCA_OFFSET_DB {
        "ride"
    } else {
        "mute"
    };
    session::Automation {
        id: id(&format!("{vca}-{suffix}")),
        target: address(
            Owner::Vca(id(vca)),
            Section::Own,
            control,
            ParameterChannel::Both,
        ),
        segments,
        offset: session::AutomationOffset::default(),
    }
}

/// #1655: a flat `offset_db` ride of `vca` at `db` over the run.
fn vca_flat_ride(vca: &str, db: f32) -> session::Automation {
    let end = u64::from(QUANTUM) * BLOCKS;
    vca_entry(
        vca,
        VCA_OFFSET_DB,
        vec![vca_piece(
            session::AutomationShape::Linear,
            (0, end),
            (db, db),
            session::ParameterUnit::Db,
        )],
    )
}

/// #1655: a `mute` entry of `vca` stepping from `from` at each `(sample, value)` of `steps`.
fn vca_mute_steps(vca: &str, from: f32, steps: &[(u64, f32)]) -> session::Automation {
    let mut at = (0_u64, from);
    let segments = steps
        .iter()
        .map(|&(sample, value)| {
            let piece = vca_piece(
                session::AutomationShape::Step,
                (at.0, sample),
                (at.1, value),
                session::ParameterUnit::Linear,
            );
            at = (sample, value);
            piece
        })
        .collect();
    vca_entry(vca, VCA_MUTE, segments)
}

/// #1655 gate 2 on both hosts. A linear `offset_db` ride of `v` (0 to -12 dB over the run) over
/// `t2` (own ride) and `t3` (behind the true-peak limiter) renders the same bits on both hosts in
/// every block, and is heard (a static `v` at 0 dB renders otherwise). Each member's exact gain at
/// its own node time is host-core's gate (`stored_vca_automation`,
/// `a_vca_ride_reaches_each_member_in_its_own_node_time`), which both hosts run.
///
/// Test value: red if either host prepares the VCA ride otherwise than the shared preparation
/// (another member arrival, another composition), or does not render it at all.
#[test]
fn a_linear_vca_ride_renders_alike_on_both_hosts() {
    const RUN: u64 = 32;
    let end = u64::from(QUANTUM) * BLOCKS;
    let riding = automated_vca_model(48_000, |model| {
        model.automation.push(vca_entry(
            VCA.id,
            VCA_OFFSET_DB,
            vec![vca_piece(
                session::AutomationShape::Linear,
                (0, end),
                (0.0, -12.0),
                session::ParameterUnit::Db,
            )],
        ));
    });
    let ridden = both_run(&riding, RUN, &[]);
    let still = both_run(&automated_vca_model(48_000, |_| {}), RUN, &[]);
    assert!(
        (8..RUN as usize).all(|block| ridden[block] != still[block]),
        "the ride is heard"
    );
}

/// #1655 gate 3's session on both hosts: [`follow_model`]`(false, false)` with a true-peak limiter
/// on `t1` (so `t1-b`'s `pre_fader` tap and `t1`'s fader sit `L` behind render time), `t1` in VCA
/// `n` (no own mute entry), `n` nested in `v`, which also holds `t2`. With `curve`, `v`'s mute
/// closes at 1,000 and opens at 2,500; with `muted`, `v` is muted statically.
fn vca_follow_model(curve: bool, muted: bool) -> SessionModel {
    let limiter = model_at(48_000).tracks[2].inserts.effects[LIMITER.position as usize].clone();
    let mut model = follow_model(false, false);
    model
        .tracks
        .iter_mut()
        .find(|track| track.id.as_str() == T1.id)
        .expect("t1")
        .inserts
        .effects
        .push(limiter);
    let mut v = vca_at(VCA.id, 0.0, &[T2.id, "n"]);
    (v.fader.left_mute, v.fader.right_mute) = (muted, muted);
    model.vcas = vec![vca_at("n", 0.0, &[T1.id]), v];
    if curve {
        model
            .automation
            .push(vca_mute_steps(VCA.id, 0.0, &[(1_000, 1.0), (2_500, 0.0)]));
    }
    model
}

/// #1655 gate 3 and the V4 note on both hosts. `v`'s mute curve reaches `t1` through the nested
/// VCA `n`; `t1` has no mute entry of its own, so its following send `t1-b` follows a lane that
/// only a VCA mute reaches (it prepares the seven-word cell and renders), and its tap sits behind
/// the limiter. Both hosts render the same bits. From the first block boundary after the mute
/// ramp's end in the output (1,000 plus the plan latency plus the 480-sample `VcaMute` ramp) to
/// the unmute, each host's output equals the plan with `v` muted statically; from the first
/// boundary after the unmute ramp, the plan with no VCA mute. Not vacuous: before the mute the
/// curve's plan differs from the statically muted one.
///
/// Test value: red if a following send of a member keeps passing its pre-fader audio into `b`
/// under an automated VCA mute (the send ignores the VCA curve, or follows it on the render clock
/// instead of its tap's node time), if a nested member misses the curve, if a VCA-only-lane send
/// is refused, or if the hosts disagree.
#[test]
fn a_vca_mute_curve_closes_a_following_send_at_its_latent_tap_on_both_hosts() {
    const BLOCKS: u64 = 40;
    let quantum = u64::from(QUANTUM);
    let curve_model = vca_follow_model(true, false);
    let latency = browser_plan_latency(&mut Rig::new(&curve_model)).0;
    assert!(latency > 0, "the limiter delays the output");
    let curve = both_run(&curve_model, BLOCKS, &[]);
    let muted = both_run(&vca_follow_model(false, true), BLOCKS, &[]);
    let open = both_run(&vca_follow_model(false, false), BLOCKS, &[]);
    let first_after = |sample: u64| sample.div_ceil(quantum);
    let closed = first_after(1_000 + latency + 480)..2_500 / quantum;
    assert!(!closed.is_empty());
    for block in closed {
        assert_eq!(
            curve[block as usize], muted[block as usize],
            "block {block}: the statically muted plan's bits while the VCA curve mutes"
        );
    }
    for block in first_after(2_500 + latency + 480)..BLOCKS {
        assert_eq!(
            curve[block as usize], open[block as usize],
            "block {block}: the open plan's bits after the unmute"
        );
    }
    assert_ne!(
        curve[4], muted[4],
        "the members and the send play before the mute"
    );
}

/// #1655 gate 4's session: [`automated_vca_model`] with `v` riding its `offset_db` (0 to -12 dB
/// over the run) and a flat `mute` entry at `mute` (0 open, 1 closed).
fn vca_hand_model(mute: f32) -> SessionModel {
    let end = u64::from(QUANTUM) * BLOCKS;
    automated_vca_model(48_000, |model| {
        model.automation.extend([
            vca_entry(
                VCA.id,
                VCA_OFFSET_DB,
                vec![vca_piece(
                    session::AutomationShape::Linear,
                    (0, end),
                    (0.0, -12.0),
                    session::ParameterUnit::Db,
                )],
            ),
            vca_mute_steps(VCA.id, mute, &[(end, mute)]),
        ]);
    })
}

/// #1655 gate 4 on both hosts (D5 and root's ruling). On [`vca_hand_model`], each edit commits
/// with the same verdict and reply on both hosts, both render the same bits, and from the end of
/// its jump each renders the committed session's boot:
/// - `SET_CONTROL offset_db = +3` on the automated `v` is `live` and heard (the stored offset);
/// - a hand mute of `v` is `live`: over an open curve it closes the members (heard), over a closed
///   curve nothing moves (the OR);
/// - a hand fader move of `t3`, which only `v`'s ride reaches, is `live` and heard (its `own_db`);
/// - adding `t1` to the automated `v` rebuilds and completes `EXACT` with no transition fallback
///   on both hosts; the same edit of a `v` no entry drives is `live` and heard.
///
/// Test value: red if a hand move of an automated VCA writes the fallback (or commits
/// `model_only`), if a hand VCA mute replaces the curve instead of ORing with it, if a plain
/// member's hand move is lost under the VCA curve, if an automated VCA's reach change commits live
/// (no event state for the new pair) or completes otherwise than `EXACT`, or if the hosts disagree.
#[test]
fn hand_edits_of_an_automated_vca_commit_as_d5_says_on_both_hosts() {
    let open = vca_hand_model(0.0);
    let closed = vca_hand_model(1.0);
    let lengths = open.control_smoothing_samples();
    let vca_mute = own_edit(Owner::Vca(id(VCA.id)), VCA_MUTE, 1.0);
    let t3_fader = SessionEdit::set_control(
        address(
            Owner::Strip(id(T3.id)),
            Section::Builtins,
            FADER_DB,
            Lane::Both,
        ),
        -6.0,
        None,
    );
    let cases: [(&str, &SessionModel, SessionEdit, u32, bool); 4] = [
        (
            "offset_db = +3",
            &open,
            vca_offset(VCA.id, 3.0, None),
            lengths.fader,
            true,
        ),
        (
            "a hand mute over an open curve",
            &open,
            vca_mute.clone(),
            lengths.mute,
            true,
        ),
        (
            "a hand mute over a closed curve",
            &closed,
            vca_mute,
            lengths.mute,
            false,
        ),
        ("a hand move of t3", &open, t3_fader, lengths.fader, true),
    ];
    for (label, model, edit, jump, heard) in cases {
        automated_vca_alike(label, model, &[edit], EditPath::Live, jump, heard);
    }
    let reach = SessionEdit::UpsertVca {
        vca: vca_at(VCA.id, 0.0, &[T1.id, T2.id, T3.id]),
    };
    // The same edit of a `v` at -3 dB that no entry drives: `t1` hears it.
    automated_vca_alike(
        "a membership edit of a VCA no entry drives",
        &automated_vca_model(48_000, |model| {
            model.vcas = vec![vca_at(VCA.id, -3.0, &[T2.id, T3.id])];
        }),
        &[SessionEdit::UpsertVca {
            vca: vca_at(VCA.id, -3.0, &[T1.id, T2.id, T3.id]),
        }],
        EditPath::Live,
        lengths.fader,
        true,
    );
    let label = "a membership edit of the automated VCA";
    let mut browser = Rig::new(&open);
    let mut c_abi = CAbi::new(&open);
    same_blocks(&mut browser, &mut c_abi, 0..4, label);
    both_commit(
        &mut browser,
        &mut c_abi,
        std::slice::from_ref(&reach),
        EditPath::Rebuild,
        label,
    );
    same_blocks(&mut browser, &mut c_abi, 4..12, label);
    let watermark = c_abi.watermark();
    assert_eq!(
        status_watermark(&browser),
        watermark,
        "{label}: one watermark"
    );
    assert_eq!(
        (watermark.0, watermark.2),
        (open.revision + 1, capi::OUTCOME_EXACT),
        "{label}: the rebuild completes exactly"
    );
    assert_eq!(browser.host.read_watermark(), host_web::RESULT_OK);
    assert_eq!(
        (
            browser.host.watermark().transition_fallback_count,
            c_abi.transition_fallback_count()
        ),
        (0, 0),
        "{label}: no transition fallback on either host"
    );
}

/// #1655 gate 4 (R17-1) on both hosts: `SET_CONTROL offset_db = +3` on the automated `v` of
/// [`vca_hand_model`] commits in place on both hosts, and the committed document stores `+3` as
/// the ride's offset on both fields with `v`'s static offset kept. The same transaction by the full
/// path (`control_plane::test_force_full_preparation`) gives the same reply, document and bits of
/// every block, alike on both hosts.
///
/// Test value: red if the in-place path answers or renders otherwise than the full path for an
/// automated VCA offset (an owners model without the VCA entry writes the static offset or commits
/// `model_only`), or if the two hosts disagree.
#[cfg(feature = "test-support")]
#[test]
fn a_hand_move_on_an_automated_vca_offset_is_in_place_as_the_full_path_on_both_hosts() {
    const EDIT: u64 = 4;
    const END: u64 = 20;
    let model = vca_hand_model(0.0);
    let mut expected = model.clone();
    expected.revision = model.revision + 1;
    expected
        .automation
        .iter_mut()
        .find(|entry| entry.id.as_str() == "v-ride")
        .expect("the ride")
        .offset = session::AutomationOffset {
        left_db: 3.0,
        right_db: 3.0,
    };
    let edits = [vca_offset(VCA.id, 3.0, None)];
    let leg = |label: &str| {
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        same_blocks(&mut browser, &mut c_abi, 0..EDIT, label);
        let (b, reply) = browser.transaction_reply(&edits);
        let (c, c_reply) = c_abi.transaction(&edits);
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
        let committed = snapshot_bytes(|payload, expected| browser.command(payload, expected));
        let blocks: Vec<Vec<u32>> = (EDIT..END)
            .map(|block| {
                let (b, c) = (browser.step(block), c_abi.step(block));
                assert_eq!(b, c, "{label}: block {block} alike on both hosts");
                b
            })
            .collect();
        (reply, committed, blocks)
    };
    control_plane::test_take_in_place_counts();
    let in_place = leg("in place");
    assert_eq!(
        control_plane::test_take_in_place_counts(),
        (2, 0),
        "both hosts commit the hand move in place"
    );
    assert_eq!(
        in_place.1,
        canonical_session_json(&expected)
            .expect("canonical")
            .as_bytes(),
        "the ride stores +3 on both fields and the static offset stays"
    );
    control_plane::test_force_full_preparation(true);
    let full = leg("full path");
    control_plane::test_force_full_preparation(false);
    assert_eq!(
        control_plane::test_take_in_place_counts().0,
        0,
        "the full leg commits nothing in place"
    );
    assert_eq!(full.0, in_place.0, "one reply on both paths");
    assert_eq!(full.1, in_place.1, "one document");
    assert_eq!(full.2, in_place.2, "the same bits on both paths");
}

/// #1655 gate 7 on both hosts (#1711's deferred jump on the composed state). [`model_at`] with
/// `v` holding `t2` and `t3` (neither automated on its own) and riding its `offset_db` linearly
/// from 0 to -12 dB up to 1,000,000. A live level move with its own 4,000 samples (a hand move of
/// `t3`'s fader, its `own_db`; a hand move of `v`'s offset, the ride's offset), heard 1, 12 and 26
/// blocks, then a rebuild whose only change is the ride's table (a segment appended after
/// 1,000,000: every reached member channel repositions, and no heard value moves). Both hosts
/// give one verdict and reply per edit and render the same bits; every block after the rebuild
/// equals, on each host, the same host's twin that took the move and no rebuild (the move keeps
/// its length); the rebuild completes `EXACT` with no transition fallback.
///
/// Test value: red if a repositioned composed channel cuts the move in flight at the swap (the
/// landing jumps to the curve over the fader length instead of deferring to the move's first
/// free sample), if either host carries the composed state differently, or if the rebuild is
/// flagged or reported otherwise than `EXACT`.
#[test]
fn a_vca_ride_table_edit_while_a_level_move_ramps_completes_exactly_on_both_hosts() {
    let ride = |appended: bool| {
        let mut segments = vec![vca_piece(
            session::AutomationShape::Linear,
            (0, 1_000_000),
            (0.0, -12.0),
            session::ParameterUnit::Db,
        )];
        if appended {
            segments.push(vca_piece(
                session::AutomationShape::Linear,
                (1_000_000, 2_000_000),
                (-12.0, -6.0),
                session::ParameterUnit::Db,
            ));
        }
        vca_entry(VCA.id, VCA_OFFSET_DB, segments)
    };
    let mut model = model_at(48_000);
    model.vcas = vec![vca_at(VCA.id, 0.0, &[T2.id, T3.id])];
    model.automation.push(ride(false));
    let t3_move = CAbi::edits(
        Edit::Fader {
            strip: T3,
            lanes: Lanes::Both,
            db: -6.0,
        },
        Some(4_000),
    );
    let moves = [
        ("a hand move of t3", t3_move),
        (
            "a hand move of v",
            vec![vca_offset(VCA.id, -6.0, Some(4_000))],
        ),
    ];
    for (what, edits) in &moves {
        for heard in [1_u64, 12, 26] {
            let label = &format!("{what}, heard {heard}");
            let mut browser = Rig::new(&model);
            let mut c_abi = CAbi::new(&model);
            let mut twin = Rig::new(&model);
            let mut c_twin = CAbi::new(&model);
            for block in 0..4 {
                let b = browser.step(block);
                assert_eq!(b, c_abi.step(block), "{label}: block {block}");
                assert_eq!(b, twin.step(block), "{label}: block {block}");
                c_twin.step(block);
            }
            both_commit(&mut browser, &mut c_abi, edits, EditPath::Live, label);
            both_commit(&mut twin, &mut c_twin, edits, EditPath::Live, label);
            let mut block = 4;
            let mut lockstep = |browser: &mut Rig,
                                c_abi: &mut CAbi,
                                twin: &mut Rig,
                                c_twin: &mut CAbi,
                                blocks: u64| {
                for _ in 0..blocks {
                    let (b, c) = (browser.step(block), c_abi.step(block));
                    assert_eq!(b, c, "{label}: block {block} alike on both hosts");
                    assert_eq!(
                        b,
                        twin.step(block),
                        "{label}: block {block}, the browser twin"
                    );
                    assert_eq!(
                        c,
                        c_twin.step(block),
                        "{label}: block {block}, the C ABI twin"
                    );
                    block += 1;
                }
            };
            lockstep(&mut browser, &mut c_abi, &mut twin, &mut c_twin, heard);
            let rebuild = [SessionEdit::UpsertAutomation {
                automation: {
                    let mut automation = ride(true);
                    if *what == "a hand move of v" {
                        automation.offset = session::AutomationOffset {
                            left_db: -6.0,
                            right_db: -6.0,
                        };
                    }
                    automation
                },
            }];
            both_commit(&mut browser, &mut c_abi, &rebuild, EditPath::Rebuild, label);
            lockstep(&mut browser, &mut c_abi, &mut twin, &mut c_twin, 8);
            let watermark = c_abi.watermark();
            assert_eq!(
                status_watermark(&browser),
                watermark,
                "{label}: one watermark"
            );
            assert_eq!(
                watermark.2,
                capi::OUTCOME_EXACT,
                "{label}: the outcome flags"
            );
            assert_eq!(browser.host.read_watermark(), host_web::RESULT_OK);
            assert_eq!(
                (
                    browser.host.watermark().transition_fallback_count,
                    c_abi.transition_fallback_count()
                ),
                (0, 0),
                "{label}: no transition fallback on either host"
            );
        }
    }
}

/// #1655 gate 8 on both hosts (#1651 U1 fold 1 for the composed mute). On [`follow_model`]`(false,
/// false)` with `v` holding `t2` and a mute curve closing at 1,500 and opening at 2,500, before
/// block 8 (the curve open) one transaction moves the close to 1,600 (a rebuild) and mutes `v` by
/// hand over `ramp` (none: the session's length; 0; 2,000). On each host every block equals a twin
/// that took only the hand mute (`live`) at the same block: the curve's change lies inside the
/// span the hand mute holds, so the hand mute lands over its own ramp across the swap. Both hosts
/// render the same bits.
///
/// Test value: red if the swap's adoption of the repositioned VCA mute cursor drops the terms
/// record's ramp word (the hand mute lands over the mute length, or twice), or if the hosts
/// disagree.
#[test]
fn a_hand_vca_mute_in_a_vca_mute_curve_rebuild_lands_over_its_own_ramp_on_both_hosts() {
    let session = |close: u64| {
        let mut model = follow_model(false, false);
        model.vcas = vec![vca_at(VCA.id, 0.0, &[T2.id])];
        model
            .automation
            .push(vca_mute_steps(VCA.id, 0.0, &[(close, 1.0), (2_500, 0.0)]));
        model
    };
    let model = session(1_500);
    let moved = session(1_600)
        .automation
        .into_iter()
        .next()
        .expect("the curve");
    for ramp in [None, Some(0), Some(2_000)] {
        let label = &format!("ramp {ramp:?}");
        let hand = SessionEdit::set_control(
            address(Owner::Vca(id(VCA.id)), Section::Own, VCA_MUTE, Lane::Both),
            1.0,
            ramp,
        );
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        let mut twin = Rig::new(&model);
        let mut c_twin = CAbi::new(&model);
        for block in 0..32 {
            if block == 8 {
                both_commit(
                    &mut browser,
                    &mut c_abi,
                    &[
                        SessionEdit::UpsertAutomation {
                            automation: moved.clone(),
                        },
                        hand.clone(),
                    ],
                    EditPath::Rebuild,
                    label,
                );
                both_commit(
                    &mut twin,
                    &mut c_twin,
                    std::slice::from_ref(&hand),
                    EditPath::Live,
                    label,
                );
            }
            let (b, c) = (browser.step(block), c_abi.step(block));
            assert_eq!(b, c, "{label}: block {block} alike on both hosts");
            assert_eq!(
                b,
                twin.step(block),
                "{label}: block {block}, the browser twin"
            );
            assert_eq!(
                c,
                c_twin.step(block),
                "{label}: block {block}, the C ABI twin"
            );
        }
    }
}

/// R19-6 on both hosts: VCA automation adds no host cap of its own; its pair memory is bank
/// payload under the existing caps, and the reach-pair cap still bounds it. [`bound_model`] with
/// the outermost of 256 nested VCAs riding its `offset_db` and stepping its `mute` (so every
/// track is a fader and mute event lane, reached by every VCA): at 64 tracks (16,384 reach pairs)
/// both hosts boot; at 65 tracks (one track, 256 pairs, over) both refuse with the core's typed
/// diagnostic `host.resource.vca_reach_pairs`, alike, not a panic.
///
/// Test value: red if VCA automation slips past the reach-pair cap on either host, panics when
/// one pair set is over it, or refuses a session at the cap that the existing caps admit.
#[test]
fn the_reach_pair_cap_bounds_vca_automation_alike_on_both_hosts() {
    let automated = |tracks: usize| {
        let mut model = bound_model(tracks, 256);
        model.automation.extend([
            vca_flat_ride("v255", -6.0),
            vca_mute_steps("v255", 0.0, &[(1_000, 1.0)]),
        ]);
        model
    };
    let at_the_cap = automated(64);
    assert_eq!(
        at_the_cap.vca_reach_pairs(u64::MAX),
        host_core::MAXIMUM_VCA_REACH_PAIRS
    );
    let (code, diagnostics) = c_abi_compile(&at_the_cap);
    assert_eq!(
        code,
        C_RESULT_OK,
        "{}",
        String::from_utf8_lossy(&diagnostics)
    );
    assert_eq!(browser_boot_refusal(&at_the_cap), None);
    let over = automated(65);
    let (code, c_diagnostics) = c_abi_compile(&over);
    assert_ne!(code, C_RESULT_OK);
    assert_eq!(
        browser_boot_refusal(&over).expect("the browser refuses"),
        c_diagnostics,
        "one refusal"
    );
    assert_eq!(
        c_diagnostics,
        b"host.resource.vca_reach_pairs\t$\n".to_vec()
    );
}

// ---------------------------------------------------------------------------------------------
// The #1669 automation fixture across one session seek (D4)
// ---------------------------------------------------------------------------------------------

/// `tests/browser-v1/automation-session.json` (the strip fader, strip mute, VCA offset and VCA
/// mute, with a following and a non-following send) runs the #1669 D2 transcript on the browser
/// host and the C ABI: 96 blocks, one plain session seek to timeline 30,000 under generation 2
/// before block 40, one chunk per source before every render. Every block's two planes have the
/// same bits.
///
/// Test value: red if the two hosts prepare the whole automation fixture differently or land its
/// session seek differently; the only other cross-host seek case automates the mute row alone, and
/// the VCA cases do not seek.
#[cfg(feature = "test-support")]
#[test]
fn the_automation_fixture_renders_alike_on_both_hosts_across_one_session_seek() {
    const BLOCKS: u64 = 96;
    const SEEK_BLOCK: u64 = 40;
    const SEEK_TIMELINE: u64 = 30_000;
    // The D2 PCM, `((f*m) mod 65536 - 32768) / 65536`, exact in `f32`, per source and lane.
    const SOURCES: [(&str, [u32; 2]); 2] =
        [("src-t1", [40_503, 30_011]), ("src-t2", [52_153, 18_749])];
    let pcm = |frame: u64, multiplier: u32| -> f32 {
        let frame = u32::try_from(frame).expect("frame fits u32");
        let residue = (frame.wrapping_mul(multiplier) & 0xffff) as i32 - 32_768;
        residue as f32 / 65_536.0
    };

    let document = include_str!("browser-v1/automation-session.json");
    let model = session::parse_session_json(document).expect("the automation fixture");
    let mut browser = Rig::boot(document.as_bytes(), model.sample_rate_hz);
    let mut c_abi = CAbi::new(&model);
    let plane = QUANTUM as usize;
    for block in 0..BLOCKS {
        if block == SEEK_BLOCK {
            assert_eq!(
                browser.host.session_seek(2, SEEK_TIMELINE, None),
                host_web::RESULT_OK,
                "browser session seek"
            );
            c_abi.session_seek(2, SEEK_TIMELINE, None);
        }
        let (generation, start) = if block < SEEK_BLOCK {
            (1, block * u64::from(QUANTUM))
        } else {
            (2, SEEK_TIMELINE + (block - SEEK_BLOCK) * u64::from(QUANTUM))
        };
        for (source, multipliers) in SOURCES {
            let [left, right] = multipliers.map(|multiplier| {
                (start..start + u64::from(QUANTUM))
                    .map(|frame| pcm(frame, multiplier))
                    .collect::<Vec<f32>>()
            });
            assert_eq!(
                browser.host.submit_source(
                    source.as_bytes(),
                    generation,
                    start,
                    model.sample_rate_hz,
                    &[&left, &right],
                    QUANTUM,
                    false,
                ),
                host_web::RESULT_OK,
                "block {block}: browser source {source}"
            );
            c_abi.submit_planes(source, generation, start, [&left, &right]);
        }
        assert_eq!(
            browser.host.render_next(),
            host_web::RESULT_OK,
            "block {block}: browser render"
        );
        let b = bits(browser.host.output_pcm().expect("browser output"));
        let c = c_abi.render(block);
        assert_eq!(b[..plane], c[..plane], "block {block}: left plane bits");
        assert_eq!(b[plane..], c[plane..], "block {block}: right plane bits");
    }
}

impl CAbi {
    /// Submit one command frame with `document` beside it, through
    /// `miso_engine_v1_submit_document_command` (#1386 D7), and answer as [`Self::send_raw`] does.
    fn send_document_raw(&mut self, bytes: &[u8], document: &[u8]) -> (u32, Vec<u8>) {
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
            capi::miso_engine_v1_submit_document_command(
                self.session,
                bytes.as_ptr(),
                bytes.len() as u64,
                document.as_ptr(),
                document.len() as u64,
                &mut out,
            )
        };
        response.truncate(out.required_bytes as usize);
        if code != C_RESULT_OK {
            response = self.last_error();
        }
        self.drain_reliable();
        (code, response)
    }
}

/// #1697 gate 1 (#1386 gate 6): the same `SESSION_REPLACE` frames and documents through the C
/// ABI's `miso_engine_v1_submit_document_command` and the browser's document entry
/// (`control_command_with_document`, which `miso_engine_web_v1_control_command` runs with the
/// document `miso_engine_web_v1_replace_document_ptr` staged): a rebuild, a live change, the
/// identical document (`Unchanged`) and a refusal (a digest that is not the document's). Both
/// hosts advertise the command, answer the same reply bytes (one request ID, so the whole frame),
/// keep the same control rows and `SESSION_SNAPSHOT_GET` bytes, and render the same bits.
///
/// Test value: red if the browser does not pass the document beside the frame (it then answers
/// `session.replace.document_length`), does not advertise the command, or runs a replace of its
/// own that answers other bytes, commits another revision or renders other blocks than the C ABI.
#[test]
fn a_session_replace_answers_and_renders_alike_on_both_hosts() {
    use sha2::{Digest, Sha256};
    for rate in [44_100, 48_000] {
        let model = model_at(rate);
        let mut browser = Rig::new(&model);
        let mut c_abi = CAbi::new(&model);
        same_blocks(&mut browser, &mut c_abi, 0..2, "before the replaces");
        // Each replace is one frame both hosts receive byte for byte: one request ID above both
        // hosts' frontiers (the browser's encoder is advanced to it, the C ABI's allocator set to
        // it), so their later requests stay new.
        let replace = |browser: &mut Rig,
                       c_abi: &mut CAbi,
                       document: &[u8],
                       digest: [u8; 32],
                       label: &str|
         -> Vec<u8> {
            let payload = || {
                CommandPayload::SessionReplace(protocol::SessionReplaceRequest {
                    document_bytes: u32::try_from(document.len()).expect("length"),
                    document_sha256: digest,
                })
            };
            let expected = ExpectedRevision::Exact(SessionRevision(browser.revision));
            let frame = loop {
                let frame = browser.encode(payload(), expected);
                let request = u64::from_le_bytes(frame[24..32].try_into().expect("request ID"));
                if request > c_abi.request {
                    c_abi.request = request;
                    break frame;
                }
            };
            let length = u32::try_from(frame.len()).expect("frame length");
            browser
                .host
                .control_request_mut(length)
                .expect("the control request staging")[..frame.len()]
                .copy_from_slice(&frame);
            let b_code = browser.host.control_command_with_document(length, document);
            let b_reply = browser.host.control_response().to_vec();
            drain_events(&mut browser.host);
            let (c_code, c_reply) = c_abi.send_document_raw(&frame, document);
            assert_eq!(
                (b_code, c_code),
                (0, C_RESULT_OK),
                "{rate} Hz {label}: both entries answer"
            );
            assert_eq!(b_reply, c_reply, "{rate} Hz {label}: the same reply bytes");
            let mut fields = [0_u16; 64];
            if let DecodedTypedResponseFrame::Success { header, .. } = ProtocolCodec::default()
                .decode_typed_response(&b_reply, &mut DecodeScratch::new(&mut fields))
                .expect("a typed reply")
            {
                browser.revision = header.revision.0;
                c_abi.revision = header.revision.0;
            }
            b_reply
        };
        let path = |reply: &[u8]| {
            let mut fields = [0_u16; 64];
            match ProtocolCodec::default()
                .decode_typed_response(reply, &mut DecodeScratch::new(&mut fields))
                .expect("a typed reply")
            {
                DecodedTypedResponseFrame::Success {
                    payload: DecodedSuccessResponsePayload::SessionReplaced(applied),
                    ..
                } => Ok(applied.edit_path),
                DecodedTypedResponseFrame::NonOk { payload, .. } => Err(payload
                    .diagnostics
                    .into_iter()
                    .map(|diagnostic| diagnostic.code)
                    .collect::<Vec<_>>()),
                DecodedTypedResponseFrame::Success { .. } => panic!("a replace reply"),
            }
        };
        let digest = |bytes: &[u8]| -> [u8; 32] { Sha256::digest(bytes).into() };

        // A rebuild: a strip's input delay moves the plan.
        let mut delayed = model.clone();
        delayed.tracks[0].builtins.left.delay_samples = 32;
        delayed.tracks[0].builtins.right.delay_samples = 32;
        let document = canonical_session_json(&delayed).expect("a valid document");
        let reply = replace(
            &mut browser,
            &mut c_abi,
            document.as_bytes(),
            digest(document.as_bytes()),
            "rebuild",
        );
        assert_eq!(
            path(&reply),
            Ok(EditPath::Rebuild),
            "{rate} Hz: the rebuild"
        );
        same_blocks(&mut browser, &mut c_abi, 2..6, "after the rebuild");

        // A live change: one fader.
        let mut faded = delayed.clone();
        faded.tracks[0].fader.left_db = -3.0;
        let document = canonical_session_json(&faded).expect("a valid document");
        let reply = replace(
            &mut browser,
            &mut c_abi,
            document.as_bytes(),
            digest(document.as_bytes()),
            "live",
        );
        assert_eq!(
            path(&reply),
            Ok(EditPath::Live),
            "{rate} Hz: the live change"
        );
        same_blocks(&mut browser, &mut c_abi, 6..10, "after the live change");

        // The identical document commits nothing.
        let committed = browser.revision;
        let reply = replace(
            &mut browser,
            &mut c_abi,
            document.as_bytes(),
            digest(document.as_bytes()),
            "unchanged",
        );
        assert_eq!(
            path(&reply),
            Ok(EditPath::Unchanged),
            "{rate} Hz: unchanged"
        );
        assert_eq!(
            browser.revision, committed,
            "{rate} Hz: unchanged keeps the revision"
        );

        // A refusal: the frame's digest is not the document's.
        let mut refused = faded.clone();
        refused.tracks[0].fader.left_db = -9.0;
        let refused = canonical_session_json(&refused).expect("a valid document");
        let reply = replace(
            &mut browser,
            &mut c_abi,
            refused.as_bytes(),
            digest(document.as_bytes()),
            "refusal",
        );
        assert_eq!(
            path(&reply),
            Err(vec![String::from("session.replace.document_digest")]),
            "{rate} Hz: the refusal"
        );

        let advertised = |reply: Vec<u8>| {
            let mut fields = [0_u16; 64];
            let DecodedTypedResponseFrame::Success {
                payload: DecodedSuccessResponsePayload::Capabilities(capabilities),
                ..
            } = ProtocolCodec::default()
                .decode_typed_response(&reply, &mut DecodeScratch::new(&mut fields))
                .expect("capabilities")
            else {
                panic!("a capabilities reply")
            };
            capabilities
                .supported_commands
                .as_chunks::<2>()
                .0
                .iter()
                .any(|id| u16::from_le_bytes([id[0], id[1]]) == 0x0013)
        };
        let get = || CommandPayload::CapabilitiesGet;
        assert!(advertised(browser.command(get(), ExpectedRevision::Any)));
        assert!(advertised(c_abi.send(get(), ExpectedRevision::Any)));
        let after_b = dump(|payload, expected| browser.command(payload, expected));
        let mut committed = faded.clone();
        committed.revision = browser.revision;
        assert_eq!(
            after_b.1,
            canonical_session_json(&committed)
                .expect("a valid document")
                .as_bytes(),
            "{rate} Hz: the snapshot is the last committed document"
        );
        assert_eq!(
            after_b,
            dump(|payload, expected| c_abi.send(payload, expected))
        );
        same_blocks(&mut browser, &mut c_abi, 10..14, "after the refusal");
    }
}

// ---------------------------------------------------------------------------------------------
// A removed source through a strip removal's phase 1 (#1325 D4)
// ---------------------------------------------------------------------------------------------

impl CAbi {
    /// One plain seek's result code, refusals included.
    fn seek_code(&self, name: &str, generation: u64, source_frame: u64) -> u32 {
        // SAFETY: The session is live and used by this thread alone; the name outlives the call.
        unsafe {
            capi::miso_engine_v1_source_seek(
                self.session,
                name.as_ptr(),
                name.len() as u64,
                generation,
                source_frame,
            )
        }
    }

    /// One submit's result code, refusals included.
    fn submit_code(&self, name: &str, generation: u64, block: u64) -> u32 {
        let [left, right] = planes(0, block);
        let pointers = [left.as_ptr(), right.as_ptr()];
        let chunk = SourceChunk {
            struct_size: SOURCE_CHUNK_SIZE,
            sample_rate_hz: self.rate,
            generation,
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
        // SAFETY: As `seek_code`; the chunk's planes and the report outlive the call.
        unsafe {
            capi::miso_engine_v1_source_submit_planar_f32(
                self.session,
                name.as_ptr(),
                name.len() as u64,
                &chunk,
                &mut report,
            )
        }
    }
}

/// #1325 D4 on both hosts: `t1` and its source are removed while `t1`, `t2` and `t3` play. Through
/// phase 1 a plain seek of the kept `src-t2` and of the removed `src-t1` is accepted on both hosts
/// (the browser resolves the consumer it prepares in the plan that renders, whose source rows are
/// not the committed session's), both sources keep feeding, and every block is alike. From the
/// adoption on, the browser's unserviced submit and both hosts' seek of `src-t1` are refused as
/// `source.id.unknown`.
///
/// Test value: red if the browser resolves a seek by the committed session's source index (the
/// removed source is unknown there, or the kept one names another consumer), if the browser's
/// unserviced feed still routes a removed source to the retiring plan after the adoption, or if
/// the two hosts route either call differently.
#[test]
fn a_removed_source_seeks_and_feeds_until_its_adoption_alike_on_both_hosts() {
    let q = u64::from(QUANTUM);
    let model = model_at(48_000);
    let removed = id(T1.id);
    let mut removal: Vec<SessionEdit> = model
        .routes
        .iter()
        .filter(|route| {
            matches!(&route.source, session::RouteSource::Track { track_id, .. } if *track_id == removed)
        })
        .map(|route| SessionEdit::RemoveRoute {
            route_id: route.id.clone(),
        })
        .collect();
    removal.push(SessionEdit::RemoveTrack { track_id: removed });
    removal.push(SessionEdit::RemoveSource {
        source_id: id("src-t1"),
    });
    let mut browser = Rig::new(&model);
    let mut c_abi = CAbi::new(&model);
    for block in 0..4 {
        assert_eq!(browser.step(block), c_abi.step(block), "block {block}");
    }
    let (b, c) = (browser.transaction(&removal), c_abi.transaction(&removal).0);
    assert_eq!(b, c, "one verdict");
    assert!(
        matches!(
            c,
            Verdict::Applied {
                path: EditPath::Rebuild,
                ..
            }
        ),
        "{c:?}"
    );

    // Phase 1: both seeks are accepted on both hosts.
    for name in ["src-t2", "src-t1"] {
        assert_eq!(
            browser.host.seek_source(name.as_bytes(), 2, 4 * q),
            host_web::RESULT_OK,
            "browser seek of {name} in phase 1: {}",
            String::from_utf8_lossy(browser.host.diagnostic())
        );
        assert_eq!(
            c_abi.seek_code(name, 2, 4 * q),
            C_RESULT_OK,
            "C ABI seek of {name}"
        );
    }
    let mut block = 4;
    loop {
        for (feed, name, generation) in [(0, "src-t1", 2), (1, "src-t2", 2), (2, "src-t3", 1)] {
            let [left, right] = planes(feed, block);
            assert_eq!(
                browser.host.submit_source(
                    name.as_bytes(),
                    generation,
                    block * q,
                    browser.rate,
                    &[&left, &right],
                    QUANTUM,
                    false,
                ),
                host_web::RESULT_OK,
                "browser {name} at block {block}"
            );
            c_abi.submit_from(name, feed, block, block, generation);
        }
        assert_eq!(browser.host.render_next(), host_web::RESULT_OK);
        let b = bits(browser.host.output_pcm().expect("browser output"));
        assert_eq!(b, c_abi.render(block), "block {block} bits");
        block += 1;
        if c_abi.watermark().0 >= c_abi.revision {
            break;
        }
        assert!(block < 64, "the removal is adopted");
    }

    // From the adoption on: the browser's unserviced feed first, before any service step.
    assert_eq!(
        browser.host.submit_source(
            b"src-t1",
            2,
            block * q,
            browser.rate,
            &[&planes(0, block)[0], &planes(0, block)[1]],
            QUANTUM,
            false,
        ),
        host_web::RESULT_INVALID_ARGUMENT,
        "the browser's feed of the removed source after the adoption"
    );
    assert_eq!(browser.host.diagnostic(), b"source.id.unknown");
    assert_eq!(
        c_abi.submit_code("src-t1", 2, block),
        capi::RESULT_INVALID_ARGUMENT
    );
    assert_eq!(
        browser.host.seek_source(b"src-t1", 3, 0),
        host_web::RESULT_INVALID_ARGUMENT
    );
    assert_eq!(browser.host.diagnostic(), b"source.id.unknown");
    assert_eq!(
        c_abi.seek_code("src-t1", 3, 0),
        capi::RESULT_INVALID_ARGUMENT
    );
    assert_eq!(c_abi.last_error(), b"source.id.unknown");
}
