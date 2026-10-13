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
use effect_contract::{PREPARED_EFFECT_TARGET_WORDS, ParameterChannel as ContractChannel};
use engine::realtime::PlanarBufferMut;
use host_core::{EQ_TARGET_CAPACITY, EQ_VALUE_COUNT, EqTargetEdit, EqTargetPreparer};
use host_web::{
    ABI_VERSION, AudioWorkletEngineHost, COMMAND_EFFECT_PARAM, COMMAND_FADER_DB, COMMAND_PAN,
    COMMAND_RECORD_BYTES, RACK_INSERTS, RESULT_OK, WebBootOptions,
};
use protocol::{
    CommandPayload, CounterId, CountersRequest, DecodeScratch, DecodedSuccessResponsePayload,
    DecodedTypedResponseFrame, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit,
    SessionRevision, TypedCommandFrame,
};
use session::{
    DualMonoFader, EffectParam, MatrixOrPan, ParameterChannel, ParameterUnit, RackName, StableId,
    canonical_session_json, parse_session_json,
};

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
/// Test value: red when one host's admission or stage composition drifts from the other's and
/// that host's own pins are re-pinned to the drift, as re-pins routinely are. Every per-host test
/// then agrees with its own host, and only this test compares the two hosts' PCM bits and counts
/// for two edits in one block (#1054's cross-host bit-identity). A drift that is not re-pinned is
/// also caught by the host's own tests. The matrix and fader readers are shared code, so a defect
/// in a reader moves both hosts alike and is for their own tests, not this one.
///
/// Red mutations (each also turns that host's own tests red): make the browser's `Both` fader
/// record write only the left cell -> the browser count is 2 and its PCM differs; make the
/// control plane's fader ramp 64 samples -> its PCM differs; add one sample to the browser's pan
/// smoothing -> its PCM differs.
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

// ---------------------------------------------------------------------------------------------
// Effect cells (#1345)
// ---------------------------------------------------------------------------------------------

/// The browser command fixture (an identity session with one parametric EQ insert whose band 1 is a
/// low shelf), with a compressor insert after the EQ. Its source is `fixture-source`.
fn effect_document() -> String {
    let mut model =
        parse_session_json(include_str!("browser-v1/command-session.json")).expect("fixture");
    model.sources[0].frames = 8_192;
    let mut compressor = model.tracks[0].inserts.effects[0].clone();
    compressor.id = StableId::parse("comp").expect("compressor ID");
    compressor.identity = session::EffectIdentity::Native {
        effect_id: StableId::parse("miso.compressor").expect("compressor identity"),
    };
    compressor.params.clear();
    model.tracks[0].inserts.effects.push(compressor);
    canonical_session_json(&model).expect("canonical effect session")
}

impl ControlPlane {
    fn new_with(document: &str) -> Self {
        let CompiledChildren { session, plan } =
            compile_children::<Adapter>(document, limits(), None).expect("control-plane compile");
        Self {
            session,
            plan,
            block: 0,
            request: 100,
            revision: 1,
        }
    }

    /// Several edits as one committed transaction.
    fn apply_all(&mut self, edits: &[SessionEdit]) {
        self.send(
            CommandPayload::SessionTransactionApply(edits),
            ExpectedRevision::Exact(SessionRevision(self.revision)),
        );
        self.revision += 1;
    }
}

fn browser_host_for(document: &str) -> AudioWorkletEngineHost {
    AudioWorkletEngineHost::boot(
        document.as_bytes(),
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

/// One compressor parameter edit as the control plane takes it.
fn compressor_edit(parameter_id: u32, channel: ParameterChannel, value: f32) -> SessionEdit {
    SessionEdit::UpsertEffectParam {
        track_id: StableId::parse("track").expect("track ID"),
        rack_name: RackName::Inserts,
        effect_id: StableId::parse("comp").expect("compressor ID"),
        param: EffectParam {
            parameter_id,
            channel,
            unit: ParameterUnit::Db,
            value,
        },
    }
}

/// One compressor parameter edit as the browser takes it: insert 1, one record in its own
/// submission. `channel` is the wire byte (0 left, 1 right, 2 both).
fn browser_compressor_edit(
    host: &mut AudioWorkletEngineHost,
    parameter_id: u32,
    channel: u8,
    value: f32,
) {
    let record =
        &mut host.command_staging_mut().expect("command staging")[..COMMAND_RECORD_BYTES as usize];
    record.fill(0);
    record[0] = u8::try_from(COMMAND_EFFECT_PARAM).expect("frozen kind");
    record[1] = RACK_INSERTS;
    record[2] = channel;
    // Track 0, effect index 1 (the compressor follows the EQ), the parameter id.
    record[8..12].copy_from_slice(&1_u32.to_le_bytes());
    record[12..16].copy_from_slice(&parameter_id.to_le_bytes());
    record[24..28].copy_from_slice(&value.to_le_bytes());
    assert_eq!(host.submit_commands(1), RESULT_OK);
}

/// One EQ band-1 gain edit on both lanes as the browser takes it: the record, the designed target
/// in the companion, and the prepared submission.
fn browser_eq_gain(host: &mut AudioWorkletEngineHost, value: f32) {
    let record =
        &mut host.command_staging_mut().expect("command staging")[..COMMAND_RECORD_BYTES as usize];
    record.fill(0);
    record[0] = u8::try_from(COMMAND_EFFECT_PARAM).expect("frozen kind");
    record[1] = RACK_INSERTS;
    record[2] = 2;
    record[12..16].copy_from_slice(&4_u32.to_le_bytes());
    record[24..28].copy_from_slice(&value.to_le_bytes());
    assert_eq!(
        host.copy_eq_target_config(0, u32::from(RACK_INSERTS), 0),
        RESULT_OK
    );
    let config = host.eq_target_config().expect("EQ target config");
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
        RATE,
        &seeds,
        &[EqTargetEdit {
            parameter_id: 4,
            channel: ContractChannel::Both,
            value,
        }],
        &mut targets,
    )
    .expect("prepared EQ target");
    assert_eq!(count, 1, "one edit touches one EQ section");
    let companion = host.prepared_companion_mut().expect("prepared companion");
    companion.fill(0);
    companion[0..4].copy_from_slice(&24_u32.to_le_bytes());
    companion[4..8].copy_from_slice(&ABI_VERSION.to_le_bytes());
    companion[8..16].copy_from_slice(&generation.to_le_bytes());
    companion[16..20].copy_from_slice(&(count as u32).to_le_bytes());
    let entry = &mut companion[24..104];
    entry[4..8].copy_from_slice(&u32::from(RACK_INSERTS).to_le_bytes());
    entry[16..24].copy_from_slice(&base_revision.to_le_bytes());
    entry[24..28].copy_from_slice(&targets[0].slot.to_le_bytes());
    entry[28..32].copy_from_slice(&2_u32.to_le_bytes());
    for (index, word) in targets[0].words.iter().enumerate() {
        let offset = 32 + index * 4;
        entry[offset..offset + 4].copy_from_slice(&word.to_le_bytes());
    }
    assert_eq!(host.submit_prepared_commands(1, 104), RESULT_OK);
}

/// Two compressor parameter edits and one EQ band-gain edit between two blocks, then eight blocks:
/// the same PCM from both hosts and the same count of replaced writes from both (decision 15,
/// #1345 gate 5).
///
/// The edits are a compressor threshold on the left lane, the same threshold again (which replaces
/// the first in the `Left` cell: one replaced write), and the EQ's band-1 gain on both lanes (a
/// `Both` target: two cells, each written once, none replaced). So both hosts count 1.
///
/// Test value: the two hosts staging effect cells differently -- the browser writing a `Both`
/// target into one channel cell, applying a replaced threshold, or counting a replaced write the
/// control plane does not -- which breaks the cross-host bit-identity of effect edits that no
/// per-host test sees, because each host agrees with itself.
///
/// Red mutation: make the browser's prepared target write only its left cell -> its PCM differs.
#[test]
fn both_hosts_drain_two_parameter_edits_and_one_eq_gain_edit_alike() {
    let document = effect_document();
    let mut control = ControlPlane::new_with(&document);
    let mut browser = browser_host_for(&document);
    let mut untouched = browser_host_for(&document);

    // The first block runs before the edits, so the lanes' cells are unwritten.
    let mut control_blocks = vec![control.step()];
    let mut browser_blocks = vec![browser_step(&mut browser, 0)];
    let mut untouched_blocks = vec![browser_step(&mut untouched, 0)];

    for threshold in [-30.0, -24.0] {
        control.apply(compressor_edit(1, ParameterChannel::Left, threshold));
        browser_compressor_edit(&mut browser, 1, 0, threshold);
    }
    control.apply_all(&[SessionEdit::UpsertEffectParam {
        track_id: StableId::parse("track").expect("track ID"),
        rack_name: RackName::Inserts,
        effect_id: StableId::parse("eq").expect("EQ ID"),
        param: EffectParam {
            parameter_id: 4,
            channel: ParameterChannel::Both,
            unit: ParameterUnit::Db,
            value: -9.0,
        },
    }]);
    browser_eq_gain(&mut browser, -9.0);
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
        1,
        "the replaced threshold write; the Both target's two cells are written once each"
    );
    assert_eq!(
        browser.status().live_values_superseded,
        control.superseded(),
        "the browser status reports the count the C ABI reports"
    );
    assert_eq!(untouched.status().live_values_superseded, 0);
}
