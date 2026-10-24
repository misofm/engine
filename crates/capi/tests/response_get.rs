//! #1573: `RESPONSE_GET` (`0x000e`) through the exported `miso_engine_v1_submit_command`.
//!
//! * **Same curves as the core.** A configuration request answers the curves and masks of
//!   `host_core::prepare_response_query` at the session's rate and quantum, bit for bit.
//! * **One refusal table.** Each refusal a configuration target can reach is its status and code,
//!   and changes neither the session nor its revision.
//! * **Capability and replay.** The registry lists `0x000e` and sets bit 14, and a replayed read
//!   returns the same bytes without preparing an owner.

#![allow(unsafe_code)]

use core::ptr;

use capi::*;
use host_core::{
    ResponseChannels, ResponseParameterOverride, ResponsePreviewGrid, ResponsePreviewLimits,
    ResponsePreviewTarget, ResponseQuery, prepare_response_query,
};
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    ExpectedRevision, ParameterChannel, ProtocolCodec, RESPONSE_BYPASS_CONTROL, RequestId,
    ResponseConfiguration, ResponseGetRequest, ResponseGrid, ResponseOverride, ResponseResult,
    ResponseTarget, StatusCode, TypedCommandFrame,
};

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const EQ: &str = "miso.parametric-eq";
const RATE: u32 = 48_000;
const QUANTUM: u32 = 128;
/// The parametric EQ's parameter IDs: band `k` (1 to 4) owns six IDs from `16 * (k - 1) + 1` in
/// the order enabled, kind, frequency, gain, Q, shelf slope (the descriptor's `parameters`).
const BAND_1_ENABLED: u32 = 1;
const BAND_1_GAIN: u32 = 4;
const BAND_2_ENABLED: u32 = 17;
const BAND_2_GAIN: u32 = 20;
const BAND_3_ENABLED: u32 = 33;
const BAND_3_GAIN: u32 = 36;
/// The builtin input filters' parameter IDs.
const HPF_HZ: u32 = 3;
const LPF_HZ: u32 = 4;

fn limits(frame_bytes: u64) -> CompileLimits {
    CompileLimits {
        struct_size: COMPILE_LIMITS_SIZE,
        source_ring_frames: 1_024,
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
        maximum_capi_retained_bytes: 1 << 30,
        maximum_named_allocation_bytes: 1 << 30,
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
        maximum_control_frame_bytes: frame_bytes,
        maximum_replay_bytes: frame_bytes.max(8_192) * 16,
        maximum_replay_entries: 64,
        maximum_submixes: 0,
        maximum_vcas: 0,
        reserved: [0; 2],
    }
}

/// One compiled session and the buffers a caller keeps for it.
struct Endpoint {
    session: *mut Session,
    plan: *mut Plan,
    frame_bytes: usize,
    next_request: u64,
}

impl Endpoint {
    fn new(frame_bytes: u64) -> Self {
        Self::with_session(frame_bytes, SESSION)
    }

    fn with_session(frame_bytes: u64, document: &str) -> Self {
        let config = EngineConfig {
            struct_size: ENGINE_CONFIG_SIZE,
            abi_version: ABI_VERSION,
            reserved: [0; 4],
        };
        let mut engine = ptr::null_mut();
        let mut session = ptr::null_mut();
        let mut plan = ptr::null_mut();
        let mut diagnostic_storage = [0_u8; 4_096];
        let mut diagnostics = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: diagnostic_storage.as_mut_ptr(),
            capacity_bytes: diagnostic_storage.len() as u64,
            required_bytes: 0,
        };
        let compile_limits = limits(frame_bytes);
        // SAFETY: Every descriptor and output location remains live for the complete call.
        unsafe {
            assert_eq!(
                miso_engine_v1_engine_create(&config, &mut engine),
                RESULT_OK
            );
            assert_eq!(
                miso_engine_v1_compile_session(
                    engine,
                    document.as_ptr(),
                    document.len() as u64,
                    &compile_limits,
                    &mut diagnostics,
                    &mut session,
                    &mut plan,
                ),
                RESULT_OK,
                "{}",
                String::from_utf8_lossy(&diagnostic_storage[..diagnostics.required_bytes as usize])
            );
            miso_engine_v1_engine_destroy(engine);
        }
        Self {
            session,
            plan,
            frame_bytes: usize::try_from(frame_bytes).expect("frame bytes"),
            next_request: 1,
        }
    }

    /// Submit one encoded command and return the response frame.
    fn submit(&self, request: &[u8]) -> Vec<u8> {
        let mut response = vec![0_u8; self.frame_bytes];
        let mut output = BytesOut {
            struct_size: BYTES_OUT_SIZE,
            reserved0: 0,
            data: response.as_mut_ptr(),
            capacity_bytes: response.len() as u64,
            required_bytes: 0,
        };
        // SAFETY: The session is live and used by this thread alone; both buffers are complete.
        let code = unsafe {
            miso_engine_v1_submit_command(
                self.session,
                request.as_ptr(),
                request.len() as u64,
                &mut output,
            )
        };
        assert_eq!(code, RESULT_OK);
        response.truncate(output.required_bytes as usize);
        response
    }

    fn encode(&mut self, payload: CommandPayload<'_>) -> Vec<u8> {
        let request_id = RequestId::new(self.next_request).expect("request ID");
        self.next_request += 1;
        let mut bytes = vec![0_u8; 1 << 16];
        let length = ProtocolCodec::default()
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id,
                    expected_revision: ExpectedRevision::Any,
                    payload,
                },
                &mut bytes,
            )
            .expect("command frame");
        bytes.truncate(length);
        bytes
    }

    fn ask(&mut self, request: &ResponseGetRequest) -> Reply {
        let bytes = self.encode(CommandPayload::ResponseGet(request));
        decode(&self.submit(&bytes))
    }

    /// The canonical session snapshot and the revision the endpoint reports.
    fn state(&mut self) -> (Vec<u8>, u64) {
        let bytes = self.encode(CommandPayload::SessionSnapshotGet(
            protocol::SessionSnapshotRequest {
                offset: 0,
                maximum_bytes: u32::try_from(self.frame_bytes / 2).expect("chunk"),
            },
        ));
        let response = self.submit(&bytes);
        let mut fields = [0_u16; 1024];
        let decoded = ProtocolCodec::default()
            .decode_typed_response(&response, &mut DecodeScratch::new(&mut fields))
            .expect("snapshot response");
        let DecodedTypedResponseFrame::Success {
            header,
            payload: DecodedSuccessResponsePayload::SessionSnapshot(snapshot),
        } = decoded
        else {
            panic!("a snapshot");
        };
        assert!(snapshot.eof, "the snapshot fits one chunk");
        (snapshot.canonical_json_chunk.to_vec(), header.revision.0)
    }
}

impl Drop for Endpoint {
    fn drop(&mut self) {
        // SAFETY: These are the exact live handles returned by `miso_engine_v1_compile_session`,
        // destroyed once.
        unsafe {
            miso_engine_v1_session_destroy(self.session);
            miso_engine_v1_plan_destroy(self.plan);
        }
    }
}

enum Reply {
    Result(ResponseResult),
    Refused {
        status: StatusCode,
        codes: Vec<String>,
    },
}

fn decode(response: &[u8]) -> Reply {
    let mut fields = [0_u16; 1024];
    match ProtocolCodec::default()
        .decode_typed_response(response, &mut DecodeScratch::new(&mut fields))
        .expect("a typed response")
    {
        DecodedTypedResponseFrame::Success {
            header,
            payload: DecodedSuccessResponsePayload::ResponseResult(result),
        } => {
            assert_eq!(header.message_id, protocol::MessageId::ResponseGet);
            Reply::Result(result)
        }
        DecodedTypedResponseFrame::NonOk { header, payload } => {
            assert_eq!(header.message_id, protocol::MessageId::ResponseGet);
            Reply::Refused {
                status: header.status,
                codes: payload
                    .diagnostics
                    .iter()
                    .map(|diagnostic| {
                        assert_eq!(diagnostic.severity, protocol::DiagnosticSeverity::Error);
                        diagnostic.code.clone()
                    })
                    .collect(),
            }
        }
        DecodedTypedResponseFrame::Success { .. } => panic!("another success payload"),
    }
}

fn request(target: ResponseTarget, points: u32) -> ResponseGetRequest {
    ResponseGetRequest {
        target,
        grid: ResponseGrid::Logarithmic,
        points,
        minimum_hz: 20.0,
        maximum_hz: 20_000.0,
        channels: ParameterChannel::Both,
        sections: true,
        overrides: Vec::new(),
        configuration_id: Some(0x5eed_1573),
        sample_rate_hz: None,
        quantum_frames: None,
        maximum_prepared_bytes: None,
        maximum_state_bytes: None,
        maximum_scratch_bytes: None,
        maximum_automation_spans: None,
        maximum_reply_bytes: None,
    }
}

fn eq_target() -> ResponseTarget {
    ResponseTarget::Configuration(ResponseConfiguration::Effect {
        effect: EQ.to_owned(),
        quality: session::EffectQuality::Normal,
        link_mode: session::LinkMode::DualMono,
    })
}

fn row(control: u32, lane: ParameterChannel, value: f32) -> ResponseOverride {
    ResponseOverride {
        control,
        lane,
        value,
    }
}

fn bits(values: &[f32]) -> Vec<u32> {
    values.iter().map(|value| value.to_bits()).collect()
}

fn lane(lane: ParameterChannel) -> effect_contract::ParameterChannel {
    match lane {
        ParameterChannel::Left => effect_contract::ParameterChannel::Left,
        ParameterChannel::Right => effect_contract::ParameterChannel::Right,
        ParameterChannel::Both => effect_contract::ParameterChannel::Both,
    }
}

/// The curves `host_core` gives for the same request at the session's rate and quantum.
fn core_curves(
    target: ResponsePreviewTarget<'_>,
    channels: ResponseChannels,
    sections: bool,
) -> host_core::ResponseCurves {
    core_curves_at(RATE, 20_000.0, target, channels, sections)
}

fn core_curves_at(
    rate: u32,
    maximum_hz: f32,
    target: ResponsePreviewTarget<'_>,
    channels: ResponseChannels,
    sections: bool,
) -> host_core::ResponseCurves {
    prepare_response_query(&ResponseQuery {
        configuration_id: 0x5eed_1573,
        sample_rate_hz: rate,
        quantum_frames: QUANTUM,
        target,
        grid: ResponsePreviewGrid::Logarithmic {
            points: 96,
            minimum_hz: 20.0,
            maximum_hz,
        },
        channels,
        sections,
        limits: ResponsePreviewLimits::default(),
    })
    .expect("the core prepares it")
    .curves()
    .expect("the core evaluates it")
}

fn assert_same_curves(reply: &Reply, expected: &host_core::ResponseCurves) {
    let Reply::Result(ResponseResult::Curves(curves)) = reply else {
        panic!("expected curves");
    };
    assert_eq!(curves.sample_rate_hz, expected.sample_rate_hz);
    assert_eq!(curves.configuration_id, expected.configuration_id);
    assert_eq!(curves.floor_db.to_bits(), expected.floor_db.to_bits());
    assert_eq!(curves.bypass, expected.bypass);
    assert_eq!(curves.section_count, expected.section_count);
    assert_eq!(curves.enabled_left, expected.enabled_left);
    assert_eq!(curves.enabled_right, expected.enabled_right);
    assert_eq!(curves.retained_bytes, expected.retained_bytes);
    assert_eq!(bits(&curves.frequencies_hz), bits(&expected.frequencies_hz));
    let side = |value: &Option<Vec<f32>>| value.as_deref().map(bits);
    assert_eq!(side(&curves.left_db), side(&expected.left_db));
    assert_eq!(side(&curves.right_db), side(&expected.right_db));
    assert_eq!(
        side(&curves.sections_left_db),
        side(&expected.sections_left_db)
    );
    assert_eq!(
        side(&curves.sections_right_db),
        side(&expected.sections_right_db)
    );
}

/// Gate 2: a configuration request for the EQ (asymmetric band gains, one disabled band, sections)
/// and for the input filters (different lane cutoffs) is answered with the curves and masks of the
/// core's own query at 48 kHz and the session quantum, bit for bit, whatever channels are asked
/// for; and a `both` override of a per-lane gain gives the curves of a `left` and a `right`
/// override of the same value.
///
/// Red if the C ABI computes its curves anywhere but in the core, at another rate or quantum, or
/// with the lane rule before decision 17.
#[test]
fn a_configuration_request_answers_the_curves_of_the_core() {
    let mut endpoint = Endpoint::new(1 << 20);
    let overrides = [
        row(BAND_1_GAIN, ParameterChannel::Left, 6.0),
        row(BAND_1_GAIN, ParameterChannel::Right, -4.0),
        row(BAND_2_ENABLED, ParameterChannel::Both, 1.0),
        row(BAND_2_GAIN, ParameterChannel::Left, -3.0),
        row(BAND_2_GAIN, ParameterChannel::Right, 5.0),
        row(BAND_3_ENABLED, ParameterChannel::Both, 0.0),
        row(BAND_3_GAIN, ParameterChannel::Both, 7.0),
    ];
    let core_rows: Vec<_> = overrides
        .iter()
        .flat_map(|row| {
            // The core's preview takes one row per lane of a per-lane parameter.
            match row.lane {
                ParameterChannel::Both => {
                    [ParameterChannel::Left, ParameterChannel::Right].to_vec()
                }
                single => vec![single],
            }
            .into_iter()
            .map(move |channel| ResponseParameterOverride {
                parameter_id: row.control,
                channel: lane(channel),
                value: row.value,
            })
        })
        .collect();
    for (channels, wire) in [
        (ResponseChannels::Both, ParameterChannel::Both),
        (ResponseChannels::Left, ParameterChannel::Left),
        (ResponseChannels::Right, ParameterChannel::Right),
    ] {
        for sections in [true, false] {
            let mut query = request(eq_target(), 96);
            query.overrides = overrides.to_vec();
            query.channels = wire;
            query.sections = sections;
            let reply = endpoint.ask(&query);
            let expected = core_curves(
                ResponsePreviewTarget::Effect {
                    effect_id: EQ,
                    overrides: &core_rows,
                    quality: effect_contract::EffectQuality::Normal,
                    bypass: false,
                    link_mode: effect_contract::LinkMode::DualMono,
                },
                channels,
                sections,
            );
            assert_same_curves(&reply, &expected);
            // The disabled band shows in the masks.
            assert_ne!(expected.enabled_left & 0b100, 0b100);
        }
    }

    // The bypass is the effect-wide switch.
    let mut bypassed = request(eq_target(), 96);
    bypassed.overrides = vec![row(RESPONSE_BYPASS_CONTROL, ParameterChannel::Both, 1.0)];
    let Reply::Result(ResponseResult::Curves(curves)) = endpoint.ask(&bypassed) else {
        panic!("curves");
    };
    assert!(curves.bypass);

    // Input filters with different lane cutoffs.
    let mut filters = request(
        ResponseTarget::Configuration(ResponseConfiguration::InputFilters),
        96,
    );
    filters.overrides = vec![
        row(HPF_HZ, ParameterChannel::Left, 80.0),
        row(HPF_HZ, ParameterChannel::Right, 200.0),
        row(LPF_HZ, ParameterChannel::Left, 12_000.0),
        row(LPF_HZ, ParameterChannel::Right, 8_000.0),
    ];
    let expected = core_curves(
        ResponsePreviewTarget::InputFilters {
            left_hpf_hz: 80.0,
            left_lpf_hz: 12_000.0,
            right_hpf_hz: 200.0,
            right_lpf_hz: 8_000.0,
        },
        ResponseChannels::Both,
        true,
    );
    assert_same_curves(&endpoint.ask(&filters), &expected);

    // A `both` override of a per-lane gain is a `left` and a `right` override of the value.
    let curves_of = |endpoint: &mut Endpoint, overrides: Vec<ResponseOverride>| {
        let mut query = request(eq_target(), 64);
        query.overrides = overrides;
        match endpoint.ask(&query) {
            Reply::Result(ResponseResult::Curves(curves)) => curves,
            _ => panic!("curves"),
        }
    };
    let both = curves_of(
        &mut endpoint,
        vec![
            row(BAND_1_ENABLED, ParameterChannel::Both, 1.0),
            row(BAND_1_GAIN, ParameterChannel::Both, 9.0),
        ],
    );
    let pair = curves_of(
        &mut endpoint,
        vec![
            row(BAND_1_ENABLED, ParameterChannel::Both, 1.0),
            row(BAND_1_GAIN, ParameterChannel::Left, 9.0),
            row(BAND_1_GAIN, ParameterChannel::Right, 9.0),
        ],
    );
    assert_eq!(both, pair);
    let flat = curves_of(&mut endpoint, Vec::new());
    assert_ne!(both.left_db, flat.left_db, "the override took effect");
}

/// A section enabled on the left lane alone reaches the left mask and not the right one, and the
/// curves of the lane that has it differ from the lane that does not; and at 44.1 and 96 kHz the
/// reply is the core's at the session's own rate, with the axis limit that rate's Nyquist. The
/// shared 48 kHz fixture cannot tell a left/right swap in the masks (every override there is
/// symmetric) or a hard-coded rate.
///
/// Red if the reply swaps the lane masks, enables a one-lane override on both lanes, or serves
/// the 48 kHz rate to a session at another rate.
#[test]
fn a_one_lane_section_and_another_session_rate_reach_the_reply() {
    let mut endpoint = Endpoint::new(1 << 20);
    let flat = {
        let reply = endpoint.ask(&request(eq_target(), 96));
        let Reply::Result(ResponseResult::Curves(curves)) = reply else {
            panic!("curves");
        };
        curves
    };
    let band_3 = 0b100;
    assert_eq!(
        (flat.enabled_left & band_3, flat.enabled_right & band_3),
        (0, 0),
        "band 3 starts disabled on both lanes"
    );
    let mut left_only = request(eq_target(), 96);
    left_only.overrides = vec![
        row(BAND_3_ENABLED, ParameterChannel::Left, 1.0),
        row(BAND_3_GAIN, ParameterChannel::Left, 9.0),
    ];
    let Reply::Result(ResponseResult::Curves(curves)) = endpoint.ask(&left_only) else {
        panic!("curves");
    };
    assert_eq!(curves.enabled_left & band_3, band_3, "the left mask has it");
    assert_eq!(curves.enabled_right & band_3, 0, "the right mask does not");
    assert_eq!(curves.enabled_right, flat.enabled_right);
    assert_ne!(curves.left_db, flat.left_db, "the left curve moved");
    assert_eq!(curves.right_db, flat.right_db, "the right curve did not");
    let left_rows = [
        ResponseParameterOverride {
            parameter_id: BAND_3_ENABLED,
            channel: effect_contract::ParameterChannel::Left,
            value: 1.0,
        },
        ResponseParameterOverride {
            parameter_id: BAND_3_GAIN,
            channel: effect_contract::ParameterChannel::Left,
            value: 9.0,
        },
    ];
    let eq_with = |rows| ResponsePreviewTarget::Effect {
        effect_id: EQ,
        overrides: rows,
        quality: effect_contract::EffectQuality::Normal,
        bypass: false,
        link_mode: effect_contract::LinkMode::DualMono,
    };
    let rows = &left_rows[..];
    let expected = core_curves(eq_with(rows), ResponseChannels::Both, true);
    assert_same_curves(&Reply::Result(ResponseResult::Curves(curves)), &expected);

    for rate in [44_100_u32, 96_000] {
        let mut model = session::parse_session_json(SESSION).expect("fixture parses");
        model.sample_rate_hz = rate;
        let document = session::canonical_session_json(&model).expect("canonical session");
        let mut endpoint = Endpoint::with_session(1 << 20, &document);
        let mut query = request(eq_target(), 96);
        query.overrides = left_only.overrides.clone();
        query.maximum_hz = rate as f32 / 2.0;
        let reply = endpoint.ask(&query);
        let at_rate = core_curves_at(
            rate,
            rate as f32 / 2.0,
            eq_with(rows),
            ResponseChannels::Both,
            true,
        );
        assert_same_curves(&reply, &at_rate);
        let Reply::Result(ResponseResult::Curves(curves)) = &reply else {
            panic!("curves at {rate}");
        };
        assert_eq!(curves.sample_rate_hz, rate);
        // Nyquist of the session's rate is inside the axis; the 48 kHz fixture's would not be at
        // 44.1 kHz, so a hard-coded rate refuses or moves this reply.
        assert_eq!(
            curves.frequencies_hz.last().copied(),
            Some(rate as f32 / 2.0)
        );
        // One hertz past Nyquist is refused at this rate.
        let mut past = query.clone();
        past.maximum_hz = rate as f32 / 2.0 + 1.0;
        let Reply::Refused { codes, .. } = endpoint.ask(&past) else {
            panic!("an axis past Nyquist at {rate}");
        };
        assert_eq!(codes, ["response.grid"]);
    }
}

/// Gate 3: each refusal a configuration target can reach is its status and code, with one error
/// diagnostic; the address and strip targets are unavailable until their slices; a rate in the
/// request is refused on a session endpoint; and after every refusal the session snapshot and the
/// revision are what they were.
///
/// Red if a refusal is untyped, wrongly classed, carries another code, or changes state.
#[test]
fn every_reachable_refusal_is_typed_and_changes_nothing() {
    let mut endpoint = Endpoint::new(1 << 20);
    let before = endpoint.state();
    let eq = |mutate: &dyn Fn(&mut ResponseGetRequest)| {
        let mut value = request(eq_target(), 64);
        mutate(&mut value);
        value
    };
    let cases: Vec<(&str, ResponseGetRequest, StatusCode, &str)> = vec![
        (
            "one point",
            eq(&|r| r.points = 1),
            StatusCode::InvalidField,
            "response.points",
        ),
        (
            "4097 points",
            eq(&|r| r.points = 4097),
            StatusCode::InvalidField,
            "response.points",
        ),
        (
            "257 overrides",
            eq(&|r| r.overrides = vec![row(BAND_1_GAIN, ParameterChannel::Left, 1.0); 257]),
            StatusCode::LimitExceeded,
            "response.override.count",
        ),
        (
            "an axis past Nyquist",
            eq(&|r| r.maximum_hz = 30_000.0),
            StatusCode::InvalidField,
            "response.grid",
        ),
        (
            "an axis that does not increase",
            eq(&|r| {
                r.minimum_hz = 2_000.0;
                r.maximum_hz = 20.0;
            }),
            StatusCode::InvalidField,
            "response.grid",
        ),
        (
            "a negative minimum",
            eq(&|r| r.minimum_hz = -1.0),
            StatusCode::InvalidField,
            "response.grid",
        ),
        (
            "a logarithmic axis from zero",
            eq(&|r| {
                r.grid = ResponseGrid::Logarithmic;
                r.minimum_hz = 0.0;
            }),
            StatusCode::InvalidField,
            "response.grid",
        ),
        (
            "draft quality",
            ResponseGetRequest {
                target: ResponseTarget::Configuration(ResponseConfiguration::Effect {
                    effect: EQ.to_owned(),
                    quality: session::EffectQuality::Draft,
                    link_mode: session::LinkMode::DualMono,
                }),
                ..request(eq_target(), 64)
            },
            StatusCode::ValidationFailed,
            "effect.quality.unsupported",
        ),
        (
            "high quality",
            ResponseGetRequest {
                target: ResponseTarget::Configuration(ResponseConfiguration::Effect {
                    effect: EQ.to_owned(),
                    quality: session::EffectQuality::High,
                    link_mode: session::LinkMode::DualMono,
                }),
                ..request(eq_target(), 64)
            },
            StatusCode::ValidationFailed,
            "effect.quality.unsupported",
        ),
        (
            "an unknown effect",
            ResponseGetRequest {
                target: ResponseTarget::Configuration(ResponseConfiguration::Effect {
                    effect: "miso.no-such-effect".to_owned(),
                    quality: session::EffectQuality::Normal,
                    link_mode: session::LinkMode::DualMono,
                }),
                ..request(eq_target(), 64)
            },
            StatusCode::NotFound,
            "response.effect.unknown",
        ),
        (
            "an effect with no response",
            ResponseGetRequest {
                target: ResponseTarget::Configuration(ResponseConfiguration::Effect {
                    effect: "miso.compressor".to_owned(),
                    quality: session::EffectQuality::Normal,
                    link_mode: session::LinkMode::DualMono,
                }),
                ..request(eq_target(), 64)
            },
            StatusCode::Unavailable,
            "response.effect.unsupported",
        ),
        (
            "an unknown parameter",
            eq(&|r| r.overrides = vec![row(9_999, ParameterChannel::Both, 1.0)]),
            StatusCode::InvalidField,
            "response.override.control",
        ),
        (
            "a lane given twice",
            eq(&|r| {
                r.overrides = vec![
                    row(BAND_1_GAIN, ParameterChannel::Left, 1.0),
                    row(BAND_1_GAIN, ParameterChannel::Left, 2.0),
                ];
            }),
            StatusCode::InvalidField,
            "response.override.duplicate",
        ),
        (
            "both beside left",
            eq(&|r| {
                r.overrides = vec![
                    row(BAND_1_GAIN, ParameterChannel::Both, 1.0),
                    row(BAND_1_GAIN, ParameterChannel::Left, 2.0),
                ];
            }),
            StatusCode::InvalidField,
            "response.override.duplicate",
        ),
        (
            "a gain outside its domain",
            eq(&|r| r.overrides = vec![row(BAND_1_GAIN, ParameterChannel::Both, 500.0)]),
            StatusCode::ValidationFailed,
            "effect.parameter.initial",
        ),
        (
            "no preparation budget",
            eq(&|r| r.maximum_prepared_bytes = Some(0)),
            StatusCode::LimitExceeded,
            "response.limit",
        ),
        (
            "a prepared-byte budget the owner cannot meet",
            eq(&|r| r.maximum_prepared_bytes = Some(1)),
            StatusCode::LimitExceeded,
            "response.limit",
        ),
        (
            "a rate in the request",
            eq(&|r| r.sample_rate_hz = Some(RATE)),
            StatusCode::InvalidField,
            "response.rate.session",
        ),
        (
            "a quantum in the request",
            eq(&|r| r.quantum_frames = Some(QUANTUM)),
            StatusCode::InvalidField,
            "response.rate.session",
        ),
        (
            "a caller reply cap below the reply",
            eq(&|r| r.maximum_reply_bytes = Some(1_024)),
            StatusCode::LimitExceeded,
            "response.reply_too_large",
        ),
        (
            "the strip target",
            ResponseGetRequest {
                target: ResponseTarget::Strip("eq0".to_owned()),
                sections: false,
                ..request(eq_target(), 64)
            },
            StatusCode::Unavailable,
            "response.target.unavailable",
        ),
    ];
    for (name, query, status, code) in cases {
        let Reply::Refused {
            status: actual,
            codes,
        } = endpoint.ask(&query)
        else {
            panic!("{name}: expected a refusal");
        };
        assert_eq!(actual, status, "{name}");
        assert_eq!(codes.len(), 1, "{name}: one diagnostic");
        assert_eq!(codes[0], code, "{name}");
        assert_eq!(
            endpoint.state(),
            before,
            "{name}: a refusal changes nothing"
        );
    }
}

/// Gate 3, the size rule: on a session whose frame limit is 4096 bytes, a 4096-point request is
/// refused `response.reply_too_large` after its owner is prepared and before any curve is computed.
///
/// Red if the reply is computed first and sized after, or if an oversized reply is sent.
#[test]
fn an_oversized_reply_is_refused_before_any_curve_is_computed() {
    let mut endpoint = Endpoint::new(4_096);
    host_core::take_response_counts();
    let Reply::Refused { status, codes } = endpoint.ask(&request(eq_target(), 4_096)) else {
        panic!("expected a refusal");
    };
    assert_eq!(status, StatusCode::LimitExceeded);
    assert_eq!(codes, ["response.reply_too_large"]);
    assert_eq!(host_core::take_response_counts(), (1, 0));
    // A request that fits is served on the same endpoint.
    let mut small = request(eq_target(), 8);
    small.sections = false;
    let Reply::Result(ResponseResult::Curves(curves)) = endpoint.ask(&small) else {
        panic!("a reply that fits");
    };
    assert_eq!(curves.frequencies_hz.len(), 8);
    assert_eq!(host_core::take_response_counts(), (1, 1));
}

/// #1574 gate 1: the session-less frame entry the browser preview uses answers the payload a C ABI
/// session answers for the same request, byte for byte. This is a wire format.
#[test]
fn the_sessionless_frame_entry_answers_the_payload_of_a_session() {
    let mut endpoint = Endpoint::new(1 << 20);
    let mut query = request(eq_target(), 96);
    query.overrides = vec![
        row(BAND_1_GAIN, ParameterChannel::Left, 6.0),
        row(BAND_1_GAIN, ParameterChannel::Right, -4.0),
        row(BAND_2_ENABLED, ParameterChannel::Both, 1.0),
        row(BAND_2_GAIN, ParameterChannel::Both, -3.0),
        row(RESPONSE_BYPASS_CONTROL, ParameterChannel::Both, 0.0),
    ];
    let session_frame = {
        let bytes = endpoint.encode(CommandPayload::ResponseGet(&query));
        endpoint.submit(&bytes)
    };
    query.sample_rate_hz = Some(RATE);
    query.quantum_frames = Some(QUANTUM);
    let request_bytes = endpoint.encode(CommandPayload::ResponseGet(&query));
    let mut reply = Vec::new();
    host_core::serve_response_frame(&request_bytes, &mut reply).expect("a command frame");
    assert!(session_frame.len() > protocol::OUTER_HEADER_BYTES);
    assert_eq!(
        &reply[protocol::OUTER_HEADER_BYTES..],
        &session_frame[protocol::OUTER_HEADER_BYTES..],
    );
    // The header differs only in the revision: the same request, a revision of 0.
    let decoded = decode(&reply);
    assert!(matches!(decoded, Reply::Result(ResponseResult::Curves(_))));
}

/// #1575, the C ABI parity row: after a live `SET_CONTROL` of one band gain of `eq0`'s console
/// entry, the address of that entry is answered with the curves of the configuration request that
/// gives the committed values with that gain, at the committed revision, with no plan rebuild; and
/// 100 address requests leave the plan's resources as they were.
///
/// Red if the host answers from a stale prepared copy instead of the committed model, or if an
/// address request leaves a prepared owner behind.
#[test]
fn an_address_follows_a_live_edit_and_keeps_nothing() {
    let mut endpoint = Endpoint::new(1 << 20);
    let plan = endpoint.plan;
    // SAFETY: The plan is live for the endpoint's lifetime and the report is a complete location.
    let resources = || unsafe {
        let mut report: PlanResourceReport = core::mem::zeroed();
        report.struct_size = PLAN_RESOURCE_REPORT_SIZE;
        assert_eq!(miso_engine_v1_plan_resources(plan, &mut report), RESULT_OK);
        report
    };
    let before = resources();
    let (_, revision) = endpoint.state();
    let edit = protocol::SessionEdit::SetControl {
        expected_effect: None,
        address: protocol::control_core::ControlAddress {
            owner: protocol::control_core::Owner::Strip(
                session::StableId::parse("eq0").expect("strip"),
            ),
            section: protocol::control_core::Section::Console(
                session::StableId::parse("eq").expect("slot"),
            ),
            control: BAND_1_GAIN,
            lane: protocol::control_core::Lane::Left,
        },
        value: 2.5,
        ramp_samples: None,
    };
    // A transaction names the revision it was written against.
    let mut bytes = vec![0_u8; 1 << 12];
    let length = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(endpoint.next_request).expect("request ID"),
                expected_revision: ExpectedRevision::Exact(protocol::SessionRevision(revision)),
                payload: CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
            },
            &mut bytes,
        )
        .expect("transaction frame");
    bytes.truncate(length);
    endpoint.next_request += 1;
    let reply = endpoint.submit(&bytes);
    let mut fields = [0_u16; 64];
    let DecodedTypedResponseFrame::Success {
        header,
        payload: DecodedSuccessResponsePayload::SessionTransactionApplied(applied),
    } = ProtocolCodec::default()
        .decode_typed_response(&reply, &mut DecodeScratch::new(&mut fields))
        .expect("transaction reply")
    else {
        panic!("the live edit is refused");
    };
    assert_eq!(
        applied.edit_path,
        protocol::EditPath::Live,
        "no plan rebuild"
    );
    assert_eq!(header.revision.0, revision + 1);

    // The committed values of `eq0`'s entry with the edited gain, as configuration overrides.
    let model = session::parse_session_json(SESSION).expect("fixture");
    let mut overrides = vec![row(RESPONSE_BYPASS_CONTROL, ParameterChannel::Both, 0.0)];
    for item in &model.tracks[0].console[0].params {
        let lane = match item.channel {
            session::ParameterChannel::Left => ParameterChannel::Left,
            session::ParameterChannel::Right => ParameterChannel::Right,
            session::ParameterChannel::Both => ParameterChannel::Both,
        };
        let edited = item.parameter_id == BAND_1_GAIN && lane == ParameterChannel::Left;
        overrides.push(row(
            item.parameter_id,
            lane,
            if edited { 2.5 } else { item.value },
        ));
    }
    let expected = endpoint.ask(&ResponseGetRequest {
        overrides,
        ..request(eq_target(), 96)
    });
    let address = request(
        ResponseTarget::Address(protocol::ControlAddress {
            owner_kind: protocol::ControlOwnerKind::Strip,
            owner_id: Some("eq0".to_owned()),
            section: protocol::ControlSection::Console,
            slot_id: Some("eq".to_owned()),
            control: None,
            lane: None,
        }),
        96,
    );
    let expected = match expected {
        Reply::Result(ResponseResult::Curves(expected)) => expected,
        Reply::Refused { status, codes } => panic!("refused {status:?} {codes:?}"),
        Reply::Result(_) => panic!("the configuration is answered"),
    };
    for _ in 0..100 {
        let bytes = endpoint.encode(CommandPayload::ResponseGet(&address));
        let reply = endpoint.submit(&bytes);
        let mut fields = [0_u16; 1024];
        let DecodedTypedResponseFrame::Success {
            header,
            payload: DecodedSuccessResponsePayload::ResponseResult(ResponseResult::Curves(curves)),
        } = ProtocolCodec::default()
            .decode_typed_response(&reply, &mut DecodeScratch::new(&mut fields))
            .expect("a typed response")
        else {
            panic!("the address is answered");
        };
        assert_eq!(header.revision.0, revision + 1, "the committed revision");
        assert_eq!(curves.enabled_left, expected.enabled_left);
        assert_eq!(curves.enabled_right, expected.enabled_right);
        for (actual, wanted) in [
            (&curves.left_db, &expected.left_db),
            (&curves.right_db, &expected.right_db),
            (&curves.sections_left_db, &expected.sections_left_db),
            (&curves.sections_right_db, &expected.sections_right_db),
        ] {
            assert_eq!(actual.as_deref().map(bits), wanted.as_deref().map(bits));
        }
    }
    assert_eq!(resources(), before, "nothing is kept");
}
