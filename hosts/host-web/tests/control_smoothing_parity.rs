//! Issue #1364 gate 3: the browser host and the C ABI render the same bits for the same session
//! and the same live edits when no edit gives a ramp length.
//!
//! The browser sends its edits as commands whose ramp word is [`SESSION_DEFAULT_RAMP`]; the C ABI
//! sends them as transactions with `ramp_samples: None`. Each resolves the absent length on its
//! own side, from the session it prepared: the browser from its `LiveRamps` and per-strip pan
//! windows, the C ABI inside `classify_live_delta`. This test is the only place the two are
//! compared. It runs the launch rates 44.1, 48, 88.2 and 96 kHz, because the length is a rounding
//! of milliseconds at the session's rate, and two tracks, one whose pan carries a window of its own
//! and one whose pan carries none, because the precedence between the strip's window and the
//! table's `pan_ms` is the second thing the hosts must agree on.
//!
//! Neither host is stubbed: the browser leg is `AudioWorkletEngineHost`, the native twin of the
//! shipped module (tied to the shipped artifact by the #106 F4 native-to-simd128 session parity
//! gate), and the C ABI leg is the exported `miso_engine_v1_*` entries.

#![allow(unsafe_code)]

use core::ptr;

use capi::{
    ABI_VERSION as C_ABI_VERSION, BYTES_OUT_SIZE, BytesOut, COMPILE_LIMITS_SIZE, CompileLimits,
    ENGINE_CONFIG_SIZE, EVENT_LANE_RELIABLE, EngineConfig, PLANAR_OUTPUT_SIZE, Plan, PlanarOutput,
    RESULT_OK as C_RESULT_OK, SOURCE_CHUNK_SIZE, SUBMIT_REPORT_SIZE, Session, SourceChunk,
    SubmitReport,
};
use host_web::{
    AudioWorkletEngineHost, COMMAND_FADER_DB, COMMAND_MUTE, COMMAND_PAN, COMMAND_RECORD_BYTES,
    RACK_NOT_APPLICABLE, RESULT_OK, SESSION_DEFAULT_RAMP, WebBootOptions,
};
use protocol::{
    CommandPayload, DecodeScratch, DecodedTypedResponseFrame, ExpectedRevision, ProtocolCodec,
    RequestId, SessionEdit, SessionRevision, StatusCode, TypedCommandFrame,
};
use session::{
    DualMonoFader, MatrixOrPan, SessionModel, StableId, canonical_session_json, parse_session_json,
};

const QUANTUM: u32 = 128;
const BLOCKS: u64 = 48;
/// The track whose pan has no window of its own (`pan_ms` applies) and the one that has.
const BARE: &str = "bare";
const WINDOWED: &str = "windowed";
const WINDOW: u32 = 200;

/// Two tracks over their own sources, each routed to the output, at `rate`.
fn model_at(rate: u32) -> SessionModel {
    let mut model = parse_session_json(include_str!("browser-v1/session.json"))
        .expect("accepted identity fixture");
    model.sample_rate_hz = rate;
    model.quantum_frames = QUANTUM;
    let source = model.sources[0].clone();
    let track = model.tracks[0].clone();
    let route = model.routes[0].clone();
    model.sources.clear();
    model.tracks.clear();
    model.routes.clear();
    for (index, id) in [BARE, WINDOWED].into_iter().enumerate() {
        let mut source = source.clone();
        source.id = StableId::parse(&format!("src-{id}")).expect("source ID");
        source.frames = u64::from(QUANTUM) * (BLOCKS + 8);
        let mut track = track.clone();
        track.id = StableId::parse(id).expect("track ID");
        track.source_id = source.id.clone();
        track.fader.left_db = -2.0 - index as f32;
        track.fader.right_db = -2.0 - index as f32;
        track.matrix_or_pan = MatrixOrPan::Pan {
            left: -0.8,
            right: 0.6,
            smoothing_samples: if id == WINDOWED { WINDOW } else { 0 },
        };
        let mut route = route.clone();
        route.id = StableId::parse(&format!("{id}-main")).expect("route ID");
        let session::RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("the fixture routes a track")
        };
        *track_id = track.id.clone();
        model.sources.push(source);
        model.tracks.push(track);
        model.routes.push(route);
    }
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
    ((z >> 40) as f32 / (1_u64 << 24) as f32) * 1.6 - 0.8
}

fn planes(feed: u64, block: u64) -> [Vec<f32>; 2] {
    let first = block * u64::from(QUANTUM);
    [0, 1].map(|lane| {
        (0..u64::from(QUANTUM))
            .map(|offset| sample(feed, lane, first + offset))
            .collect()
    })
}

/// One live edit of the script, in terms both hosts can express.
#[derive(Clone, Copy)]
enum Edit {
    /// Both lanes of a track's fader.
    Fader { track: usize, db: f32 },
    /// Both lanes of a track's mute.
    Mute { track: usize, muted: bool },
    /// A track's pan, keeping whatever window the track was prepared with.
    Pan { track: usize, left: f32, right: f32 },
}

/// The script: `(block it is submitted before, edit)`. It has no per-edit length anywhere.
const SCRIPT: [(u64, Edit); 6] = [
    (3, Edit::Fader { track: 0, db: -9.0 }),
    (
        5,
        Edit::Pan {
            track: 0,
            left: -0.2,
            right: 0.7,
        },
    ),
    (
        5,
        Edit::Pan {
            track: 1,
            left: 0.3,
            right: -0.9,
        },
    ),
    (
        8,
        Edit::Mute {
            track: 1,
            muted: true,
        },
    ),
    (
        24,
        Edit::Mute {
            track: 1,
            muted: false,
        },
    ),
    (26, Edit::Fader { track: 1, db: 3.0 }),
];

/// The browser leg: native `AudioWorkletEngineHost` with commands staged as the worklet stages them.
struct Browser {
    host: AudioWorkletEngineHost,
    rate: u32,
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

    fn command(&mut self, kind: u32, channel: u8, track: usize, ramp: u32, values: [f32; 4]) {
        let staging = self.host.command_staging_mut().expect("command staging");
        let record = &mut staging[..COMMAND_RECORD_BYTES as usize];
        record.fill(0);
        record[0] = u8::try_from(kind).expect("frozen kind");
        record[1] = RACK_NOT_APPLICABLE;
        record[2] = channel;
        record[4..8].copy_from_slice(&u32::try_from(track).expect("track").to_le_bytes());
        record[16..20].copy_from_slice(&ramp.to_le_bytes());
        for (slot, value) in values.iter().enumerate() {
            record[24 + slot * 4..28 + slot * 4].copy_from_slice(&value.to_le_bytes());
        }
        assert_eq!(self.host.submit_commands(1), RESULT_OK, "browser command");
    }

    /// The edit with no ramp length given: the session's own.
    fn edit(&mut self, edit: Edit) {
        self.edit_with_ramp(edit, SESSION_DEFAULT_RAMP);
    }

    fn edit_with_ramp(&mut self, edit: Edit, ramp: u32) {
        match edit {
            Edit::Fader { track, db } => {
                self.command(COMMAND_FADER_DB, 2, track, ramp, [db, 0.0, 0.0, 0.0]);
            }
            Edit::Mute { track, muted } => {
                self.command(
                    COMMAND_MUTE,
                    2,
                    track,
                    ramp,
                    [f32::from(u8::from(muted)), 0.0, 0.0, 0.0],
                );
            }
            Edit::Pan { track, left, right } => {
                self.command(COMMAND_PAN, 255, track, ramp, [left, right, 0.0, 0.0]);
            }
        }
    }

    fn step(&mut self, block: u64) -> Vec<u32> {
        for (feed, id) in [BARE, WINDOWED].into_iter().enumerate() {
            let [left, right] = planes(feed as u64, block);
            assert_eq!(
                self.host.submit_source(
                    format!("src-{id}").as_bytes(),
                    1,
                    block * u64::from(QUANTUM),
                    self.rate,
                    &[&left, &right],
                    QUANTUM,
                    false,
                ),
                RESULT_OK,
                "browser source {id}"
            );
        }
        assert_eq!(self.host.render_next(), RESULT_OK, "browser render");
        self.host
            .output_pcm()
            .expect("browser output")
            .iter()
            .map(|sample| sample.to_bits())
            .collect()
    }
}

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
        maximum_submixes: 0,
        maximum_vcas: 0,
        reserved: [0; 2],
    }
}

/// The C ABI leg: exported entries, edits as transactions with no `ramp_samples`.
struct CAbi {
    session: *mut Session,
    plan: *mut Plan,
    model: SessionModel,
    revision: u64,
    request: u64,
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
            model: model.clone(),
            revision: model.revision,
            request: 0,
        }
    }

    fn edit(&mut self, edit: Edit) {
        let (track, edit) = match edit {
            Edit::Fader { track, db } => {
                let fader = DualMonoFader {
                    left_db: db,
                    right_db: db,
                    ..self.model.tracks[track].fader.clone()
                };
                self.model.tracks[track].fader = fader.clone();
                (
                    track,
                    SessionEdit::SetTrackFader {
                        track_id: self.model.tracks[track].id.clone(),
                        fader,
                        ramp_samples: None,
                    },
                )
            }
            Edit::Mute { track, muted } => {
                let fader = DualMonoFader {
                    left_mute: muted,
                    right_mute: muted,
                    ..self.model.tracks[track].fader.clone()
                };
                self.model.tracks[track].fader = fader.clone();
                (
                    track,
                    SessionEdit::SetTrackFader {
                        track_id: self.model.tracks[track].id.clone(),
                        fader,
                        ramp_samples: None,
                    },
                )
            }
            Edit::Pan { track, left, right } => {
                // The edit keeps the window the track was prepared with: the window belongs to the
                // session, and the browser keeps the prepared one too.
                let MatrixOrPan::Pan {
                    smoothing_samples, ..
                } = self.model.tracks[track].matrix_or_pan
                else {
                    panic!("the fixture pans")
                };
                let matrix_or_pan = MatrixOrPan::Pan {
                    left,
                    right,
                    smoothing_samples,
                };
                self.model.tracks[track].matrix_or_pan = matrix_or_pan.clone();
                (
                    track,
                    SessionEdit::SetTrackMatrixOrPan {
                        track_id: self.model.tracks[track].id.clone(),
                        matrix_or_pan,
                        ramp_samples: None,
                    },
                )
            }
        };
        let _ = track;
        self.request += 1;
        let edits = [edit];
        let mut bytes = vec![0_u8; 4_096];
        let length = ProtocolCodec::default()
            .encode_command_frame_into(
                &TypedCommandFrame {
                    request_id: RequestId::new(self.request).expect("nonzero request"),
                    expected_revision: ExpectedRevision::Exact(SessionRevision(self.revision)),
                    payload: CommandPayload::SessionTransactionApply(&edits),
                },
                &mut bytes,
            )
            .expect("transaction frame");
        bytes.truncate(length);
        let mut response = [0_u8; 4_096];
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
        assert_eq!(code, C_RESULT_OK, "C ABI transaction");
        let mut fields = [0_u16; 64];
        let decoded = ProtocolCodec::default()
            .decode_typed_response(
                &response[..out.required_bytes as usize],
                &mut DecodeScratch::new(&mut fields),
            )
            .expect("transaction response");
        let DecodedTypedResponseFrame::Success { header, .. } = decoded else {
            panic!("the transaction was refused")
        };
        assert_eq!(header.status, StatusCode::Ok);
        assert_eq!(header.revision, SessionRevision(self.revision + 1));
        self.revision += 1;
        self.drain_reliable();
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

    fn step(&mut self, block: u64) -> Vec<u32> {
        for (feed, id) in [BARE, WINDOWED].into_iter().enumerate() {
            let [left, right] = planes(feed as u64, block);
            let pointers = [left.as_ptr(), right.as_ptr()];
            let chunk = SourceChunk {
                struct_size: SOURCE_CHUNK_SIZE,
                sample_rate_hz: self.model.sample_rate_hz,
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
            let name = format!("src-{id}");
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
            assert_eq!(code, C_RESULT_OK, "C ABI source {id}");
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
        pcm.iter().map(|sample| sample.to_bits()).collect()
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

/// Whether two renders differ anywhere, as a count of differing samples.
fn differing(a: &[u32], b: &[u32]) -> usize {
    a.iter().zip(b).filter(|(x, y)| x != y).count()
}

/// Gate 3. Red if the two hosts compute different lengths for an absent ramp (a rounding, a rate or
/// a row map that disagrees), if one resolves a pan against the table where the other uses the
/// strip's window, or if the edits land at different points. The script has no per-edit length, so
/// a host that still steps for an absent length is red against its own default-table control.
#[test]
fn the_browser_and_the_c_abi_render_the_same_bits_for_an_absent_ramp() {
    for rate in [44_100, 48_000, 88_200, 96_000] {
        let model = model_at(rate);
        let mut browser = Browser::new(&model);
        let mut c_abi = CAbi::new(&model);
        // A control: the same script under hard steps differs from the ramped run, so the ramps are
        // really being rendered and the comparison below is not between two silent or stepped runs.
        let mut stepped = Vec::new();
        let mut ramped = Vec::new();
        for block in 0..BLOCKS {
            for (at, edit) in SCRIPT {
                if at == block {
                    browser.edit(edit);
                    c_abi.edit(edit);
                }
            }
            let (b, c) = (browser.step(block), c_abi.step(block));
            assert_eq!(
                differing(&b, &c),
                0,
                "{rate} Hz: block {block} differs between the browser and the C ABI"
            );
            assert!(
                b.iter().any(|bits| f32::from_bits(*bits) != 0.0) || block > 8,
                "{rate} Hz: block {block} carries signal"
            );
            ramped.push(b);
        }
        // The same script with every edit's ramp spelled 0 steps instead.
        let mut step_browser = Browser::new(&model);
        for block in 0..BLOCKS {
            for (at, edit) in SCRIPT {
                if at == block {
                    step_browser.edit_with_ramp(edit, 0);
                }
            }
            stepped.push(step_browser.step(block));
        }
        let moved = ramped
            .iter()
            .zip(&stepped)
            .filter(|(a, b)| differing(a, b) > 0)
            .count();
        assert!(
            moved >= 8,
            "{rate} Hz: only {moved} blocks differ from the stepped run, so the ramps were not heard"
        );
    }
}
