//! #1399: the browser host and the control plane the C ABI embeds drain strip cells alike.
//!
//! One session and one script of strip edits, played once through the control plane's
//! `SESSION_TRANSACTION_APPLY` and once through the browser host's command admission. The two
//! admissions write the same cells, and the two renders drain them the same way, so the PCM is
//! bit-identical and the two `live_values_superseded` counts are equal. Decision 15, D15-2.

use control_plane::{
    CompiledChildren, ControlAdapter, ControlLimits, EventLane, PlanResources, SourceSubmission,
    compile_children,
};
use engine::realtime::PlanarBufferMut;
use host_web::{
    AudioWorkletEngineHost, COMMAND_FADER_DB, COMMAND_PAN, COMMAND_RECORD_BYTES, RESULT_OK,
    WebBootOptions,
};
use protocol::{
    CommandPayload, CounterId, CountersRequest, DecodeScratch, DecodedSuccessResponsePayload,
    DecodedTypedResponseFrame, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit,
    SessionRevision, TypedCommandFrame,
};
use session::{DualMonoFader, MatrixOrPan, StableId, canonical_session_json, parse_session_json};

/// The browser harness's one-track identity session, whose source is `fixture-source`.
const SESSION_DOCUMENT: &str = include_str!("browser-v1/session.json");
const QUANTUM: usize = 128;
const RATE: u32 = 48_000;
const SOURCE: &[u8] = b"fixture-source";
/// Blocks rendered after the edits.
const BLOCKS_AFTER_EDITS: u64 = 8;
/// The pan edits' ramp, in samples: longer than a block, so the ramp spans several of them.
const PAN_SMOOTHING: u32 = 300;

/// The control plane with no adapter allocations of its own.
struct Adapter;

impl ControlAdapter for Adapter {
    type Row = PlanResources;
    const ADAPTER_ALLOCATIONS: &'static [u64] = &[];
}

/// The session document, with a source long enough for the whole script.
fn document() -> String {
    let mut model = parse_session_json(SESSION_DOCUMENT).expect("fixture session");
    model.sources[0].frames = 8_192;
    canonical_session_json(&model).expect("canonical session")
}

fn limits() -> ControlLimits {
    ControlLimits {
        source_ring_frames: 1_024,
        maximum_automation_spans_per_block: 128,
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
        maximum_control_retained_bytes: 10_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
        maximum_control_frame_bytes: 4_096,
        maximum_replay_bytes: 8_192,
        maximum_replay_entries: 16,
        maximum_submixes: 0,
        maximum_vcas: 0,
    }
}

/// A sawtooth that differs by lane and frame, so every cell's gain is audible.
fn source_sample(lane: u64, frame: u64) -> f32 {
    let phase = (frame * (3 + 2 * lane) % 97) as f32 / 97.0;
    0.8 * (2.0 * phase - 1.0)
}

fn source_chunk(start: u64) -> [Vec<f32>; 2] {
    [0, 1].map(|lane| {
        (0..QUANTUM as u64)
            .map(|offset| source_sample(lane, start + offset))
            .collect()
    })
}

/// The scripted edits, in order: two `Both` fader edits and two pan edits on the one track.
const FADER_DB: [f32; 2] = [-3.0, -7.5];
const PAN: [[f32; 2]; 2] = [[-0.8, 0.6], [-0.4, 0.9]];

/// The control plane's side of the script: a session and its plan, fed and rendered a quantum at a
/// time.
struct ControlPlane {
    session: control_plane::SessionState<Adapter>,
    plan: control_plane::PlanState<Adapter>,
    block: u64,
    request: u64,
    /// The committed session revision: the document's, then one more per transaction.
    revision: u64,
}

impl ControlPlane {
    fn new() -> Self {
        let CompiledChildren { session, plan } =
            compile_children::<Adapter>(&document(), limits(), None)
                .expect("control-plane compile");
        Self {
            session,
            plan,
            block: 0,
            request: 100,
            revision: 1,
        }
    }

    /// Feeds the next quantum of the source and renders one block.
    fn step(&mut self) -> Vec<f32> {
        let [left, right] = source_chunk(self.block * QUANTUM as u64);
        self.session
            .submit(
                SOURCE,
                SourceSubmission {
                    generation: 1,
                    start_frame: self.block * QUANTUM as u64,
                    sample_rate_hz: RATE,
                    planes: &[&left, &right],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source submission");
        let mut pcm = vec![f32::NAN; QUANTUM * 2];
        let output = PlanarBufferMut::try_new(&mut pcm, 2, QUANTUM, QUANTUM).expect("output");
        self.plan
            .render(self.block * QUANTUM as u64, output)
            .expect("render");
        self.block += 1;
        pcm
    }

    fn send(&mut self, payload: CommandPayload<'_>, expected: ExpectedRevision) -> Vec<u8> {
        self.request += 1;
        let codec = ProtocolCodec::default();
        let frame = TypedCommandFrame {
            request_id: RequestId::new(self.request).expect("nonzero request"),
            expected_revision: expected,
            payload,
        };
        let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
        let length = codec
            .encode_command_frame_into(&frame, &mut bytes)
            .expect("typed command");
        bytes.truncate(length);
        let length = self.session.command(&bytes, 4_096).expect("command");
        let response = self.session.command_response(length).to_vec();
        // The transaction's reliable events are not under test: drain them.
        while self
            .session
            .dequeue_event(EventLane::Reliable, 4_096)
            .expect("event lane")
            .is_some()
        {}
        response
    }

    /// One edit as its own committed transaction, at the revision the last one committed.
    fn apply(&mut self, edit: SessionEdit) {
        self.send(
            CommandPayload::SessionTransactionApply(&[edit]),
            ExpectedRevision::Exact(SessionRevision(self.revision)),
        );
        self.revision += 1;
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
}

/// The browser host, fed and rendered a quantum at a time.
fn browser_host() -> AudioWorkletEngineHost {
    AudioWorkletEngineHost::boot(
        document().as_bytes(),
        WebBootOptions {
            require_sample_rate_hz: RATE,
            require_quantum_frames: QUANTUM as u32,
            source_ring_frames: 1_024,
            live_control_command_queue_records: 16,
            ..WebBootOptions::explicit_defaults()
        },
    )
    .unwrap_or_else(|failure| panic!("boot: {}", String::from_utf8_lossy(failure.diagnostic())))
}

fn browser_step(host: &mut AudioWorkletEngineHost, block: u64) -> Vec<f32> {
    let [left, right] = source_chunk(block * QUANTUM as u64);
    assert_eq!(
        host.submit_source(
            SOURCE,
            1,
            block * QUANTUM as u64,
            RATE,
            &[&left, &right],
            QUANTUM as u32,
            false,
        ),
        RESULT_OK
    );
    assert_eq!(host.render_next(), RESULT_OK);
    host.output_pcm().expect("output").to_vec()
}

/// Stages one record at slot 0 and admits it as its own submission.
fn browser_command(host: &mut AudioWorkletEngineHost, kind: u32, channel: u8, values: [f32; 4]) {
    let record_bytes = COMMAND_RECORD_BYTES as usize;
    let record = &mut host.command_staging_mut().expect("command staging")[..record_bytes];
    record.fill(0);
    record[0] = u8::try_from(kind).expect("frozen kind");
    // `rack` is not applicable (255); the track is strip 0.
    record[1] = 255;
    record[2] = channel;
    let smoothing = if kind == COMMAND_PAN {
        PAN_SMOOTHING
    } else {
        0
    };
    record[16..20].copy_from_slice(&smoothing.to_le_bytes());
    for (slot, value) in values.iter().enumerate() {
        record[24 + slot * 4..28 + slot * 4].copy_from_slice(&value.to_le_bytes());
    }
    assert_eq!(host.submit_commands(1), RESULT_OK);
}

fn bits(pcm: &[f32]) -> Vec<u32> {
    pcm.iter().map(|sample| sample.to_bits()).collect()
}

/// Two fader edits and two pan edits on one track between two blocks, then eight blocks: the same
/// PCM from both hosts, and the same count of replaced writes from both.
///
/// The count is 3: the second `Both` fader edit replaces the first's value in each of the two
/// channel cells (2), and the second pan edit replaces the first's in the matrix cell (1).
///
/// Test value: red if one host applies a cell's older value, writes a `Both` record into one
/// channel cell rather than two, or counts a replaced write the other does not: each breaks
/// #1054's cross-host bit-identity for two edits in one block, which no per-host test sees,
/// because each host agrees with itself.
///
/// Red mutation: make the browser's `Both` fader record write only the left cell -> the browser
/// count is 2 and its PCM differs. Count the control plane's matrix cell as unreplaced -> 2, not 3.
#[test]
fn both_hosts_drain_two_fader_and_two_pan_edits_alike() {
    let track = StableId::parse("track").expect("track ID");
    let mut control = ControlPlane::new();
    let mut browser = browser_host();
    let mut untouched = browser_host();

    // The first block runs before the edits, so the strip's cells hold their seeds.
    let mut control_blocks = vec![control.step()];
    let mut browser_blocks = vec![browser_step(&mut browser, 0)];
    let mut untouched_blocks = vec![browser_step(&mut untouched, 0)];

    for (fader_db, pan) in FADER_DB.into_iter().zip(PAN) {
        control.apply(SessionEdit::SetTrackFader {
            track_id: track.clone(),
            fader: DualMonoFader {
                left_db: fader_db,
                right_db: fader_db,
                left_mute: false,
                right_mute: false,
            },
        });
        browser_command(&mut browser, COMMAND_FADER_DB, 2, [fader_db, 0.0, 0.0, 0.0]);
        control.apply(SessionEdit::SetTrackMatrixOrPan {
            track_id: track.clone(),
            matrix_or_pan: MatrixOrPan::Pan {
                left: pan[0],
                right: pan[1],
                smoothing_samples: PAN_SMOOTHING,
            },
        });
        browser_command(&mut browser, COMMAND_PAN, 255, [pan[0], pan[1], 0.0, 0.0]);
    }
    assert_eq!(control.superseded(), 0, "nothing has been read yet");
    assert_eq!(browser.status().live_values_superseded, 0);

    for block in 1..=BLOCKS_AFTER_EDITS {
        control_blocks.push(control.step());
        browser_blocks.push(browser_step(&mut browser, block));
        untouched_blocks.push(browser_step(&mut untouched, block));
    }

    for (block, (control_pcm, browser_pcm)) in
        control_blocks.iter().zip(&browser_blocks).enumerate()
    {
        assert_eq!(
            bits(control_pcm),
            bits(browser_pcm),
            "block {block}: the two hosts render the same bits"
        );
    }
    assert_ne!(
        bits(&browser_blocks[1]),
        bits(&untouched_blocks[1]),
        "the edits are audible, or the comparison proves nothing"
    );
    assert_eq!(
        control.superseded(),
        3,
        "two fader cells and the matrix cell"
    );
    assert_eq!(
        browser.status().live_values_superseded,
        control.superseded(),
        "the browser status reports the count the C ABI reports"
    );
    assert_eq!(untouched.status().live_values_superseded, 0);
}
