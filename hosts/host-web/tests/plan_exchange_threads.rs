//! Issue #1381a: the browser instance renders through the plan owner, adopts a published successor
//! at a block boundary, and reclaims the displaced plan and its attachment in the service step.
//!
//! The file keeps the name of the full issue's integration binary (`plan_exchange_threads`). Its
//! first slice runs the one instance of today's module (decision 17, "single instance first"), so
//! the control half and the render half are one thread, which is gate 4's form; the two-thread
//! form arrives with the Worker (#1332, #1387). Reader pairing across a swap is #1381b's, so a
//! swap here asserts the clock, the bits, the allocation counts and the reclaim, and nothing about
//! the meters', observations' or spectrum's continuity.
//!
//! Every call goes through the module's exports on a fresh thread, as the worklet makes them, so a
//! render call runs inside its render-locked window and the instance's own counters read what it
//! allocated and freed: the render-locked count must not move across a render, and the control
//! count must move across the service step that drops the displaced plan.
//!
//! Decision 15 rules that host-web's native allocation-count gates live in an integration binary,
//! never in `src/tests.rs`, which registers its own allocator. This binary registers the module's
//! own counting allocator and holds two tests that read its process-wide counters in turn.

#![cfg(feature = "test-support")]

use std::alloc::System;
use std::sync::Mutex;

use host_web::{
    RESULT_INVALID_ARGUMENT, RESULT_OK, RenderLockedAllocator, WATERMARK_BYTES, WebBootOptions,
    companion_drops_on_this_thread, native_staging,
};
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision, StatusCode,
    TypedCommandFrame,
};
use session::{RouteSource, SessionModel, StableId, canonical_session_json, parse_session_json};

#[global_allocator]
static ALLOCATOR: RenderLockedAllocator<System> = RenderLockedAllocator(System);

/// The tests read process-wide counters, so they take turns.
static TURN: Mutex<()> = Mutex::new(());

/// The nine-track fixture of the engine's identity corpus.
const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE_ID: &[u8] = b"fixture-source";
const QUANTUM_FRAMES: u32 = 128;
const BLOCKS: usize = 12;
/// The block before which the successor is published.
const PUBLISH_BEFORE: usize = 6;

/// Nine stereo tracks over one source, with both console sections empty, no insert and the input
/// filters off. A console slot, an insert and an input filter carry no state across a plan swap
/// yet (#1281, #1282, #1277 D6), so a session that holds one would not render the same bits after
/// the adoption; the host-core successor test (`successor_swap.rs`) builds its session the same
/// way.
fn document() -> String {
    let mut model = parse_session_json(FIXTURE).expect("the fixture parses");
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
        for channel in [&mut track.builtins.left, &mut track.builtins.right] {
            channel.hpf_hz = 0.0;
            channel.lpf_hz = 0.0;
        }
    }
    canonical_session_json(&model).expect("the session is canonical")
}

fn options() -> WebBootOptions {
    WebBootOptions {
        live_control_meter_blocks: 2,
        live_control_observation_taps: 4,
        ..WebBootOptions::explicit_defaults()
    }
}

/// A muted copy of the first track and its route to the output: a structural edit whose
/// successor adds exact `+0.0` to the master sum, so every block is the bits of the run without it.
fn add_muted_track(model: &SessionModel, id: &str) -> [SessionEdit; 2] {
    let mut track = model.tracks[0].clone();
    track.id = StableId::parse(id).expect("track ID");
    track.fader.left_mute = true;
    track.fader.right_mute = true;
    let mut route = model.routes[0].clone();
    route.id = StableId::parse(&format!("{id}-main")).expect("route ID");
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("a track route")
    };
    *track_id = track.id.clone();
    [
        SessionEdit::UpsertTrack { track },
        SessionEdit::UpsertRoute { route },
    ]
}

fn transaction_bytes(edits: &[SessionEdit], revision: u64) -> Vec<u8> {
    let codec = ProtocolCodec::default();
    let frame = TypedCommandFrame {
        request_id: RequestId::new(1).expect("nonzero request"),
        expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
        payload: CommandPayload::SessionTransactionApply(edits),
    };
    let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
    let length = codec
        .encode_command_frame_into(&frame, &mut bytes)
        .expect("typed command");
    bytes.truncate(length);
    bytes
}

/// Assert the control plane accepted the transaction as a rebuild, and published its plan.
fn assert_rebuild_applied(response: &[u8], revision: u64) {
    let mut fields = [0_u16; 64];
    let decoded = ProtocolCodec::default()
        .decode_typed_response(response, &mut DecodeScratch::new(&mut fields))
        .expect("a transaction response");
    let (header, payload) = match decoded {
        DecodedTypedResponseFrame::Success { header, payload } => (header, payload),
        DecodedTypedResponseFrame::NonOk { header, payload } => panic!(
            "the transaction was refused ({:?}): {:?}",
            header.status,
            payload
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.code.to_string())
                .collect::<Vec<_>>()
        ),
    };
    assert_eq!(header.status, StatusCode::Ok);
    assert_eq!(header.revision, SessionRevision(revision));
    let DecodedSuccessResponsePayload::SessionTransactionApplied(applied) = payload else {
        panic!("a transaction success")
    };
    assert_eq!(applied.edit_path, EditPath::Rebuild);
}

fn boot() -> u32 {
    let length =
        native_staging::boot(options(), document().as_bytes()).expect("the fixture stages");
    let handle = host_web::miso_engine_web_v1_boot(length);
    assert_ne!(handle, 0, "the fixture boots");
    handle
}

/// Feed one quantum of a block-dependent level to the track's source, through the export.
fn feed(handle: u32, block: usize) {
    let id = SOURCE_ID.len() as u32;
    assert!(native_staging::source_id(handle, SOURCE_ID));
    assert!(native_staging::source_pcm(
        handle,
        0.05 + 0.02 * block as f32
    ));
    let result = host_web::miso_engine_web_v1_source_submit(
        handle,
        id,
        1,
        (block as u64) * u64::from(QUANTUM_FRAMES),
        2,
        QUANTUM_FRAMES,
        0,
    );
    assert_eq!(result, RESULT_OK, "block {block}: the source feed");
}

/// What one run observed.
struct Run {
    /// The output planes of every block, as bits.
    bits: Vec<Vec<u32>>,
    /// The epoch of the plan that rendered each block.
    epochs: Vec<u64>,
    /// `next_absolute_sample` after each block.
    clock: Vec<u64>,
    /// Render-locked allocator calls over every render call: it must stay zero.
    render_allocations: u32,
    /// Attachments that dropped inside a render call, over the whole run.
    drops_in_render: u64,
    /// Attachments that dropped by the first service call after the swap block, and the control
    /// counter's movement across that call.
    drops_at_first_service: u64,
    control_calls_at_first_service: u32,
    /// Attachments that dropped over the whole run, the closing service step included.
    drops_in_total: u64,
}

/// Render `BLOCKS` blocks on a fresh thread's instance, publishing a successor before block
/// `PUBLISH_BEFORE` when `republish`, and servicing at the swap block's end, before anything else
/// touches the control plane.
fn run(republish: bool) -> Run {
    std::thread::spawn(move || {
        let handle = boot();
        let model = parse_session_json(&document()).expect("the fixture parses");
        let request = transaction_bytes(&add_muted_track(&model, "a-muted"), model.revision);
        let mut run = Run {
            bits: Vec::new(),
            epochs: Vec::new(),
            clock: Vec::new(),
            render_allocations: 0,
            drops_in_render: 0,
            drops_at_first_service: 0,
            control_calls_at_first_service: 0,
            drops_in_total: 0,
        };
        let drops_before = companion_drops_on_this_thread();
        for block in 0..BLOCKS {
            feed(handle, block);
            if republish && block == PUBLISH_BEFORE {
                assert_eq!(native_staging::active_plan_epoch(handle), Some(0));
                let (result, response) =
                    native_staging::control_command(handle, &request).expect("a live handle");
                assert_eq!(result, RESULT_OK, "the structural transaction publishes");
                assert_rebuild_applied(&response, model.revision + 1);
                assert_eq!(
                    native_staging::active_plan_epoch(handle),
                    Some(0),
                    "a published successor waits for the next block boundary"
                );
            }
            let allocations = host_web::miso_engine_web_v1_render_allocation_count();
            let drops = companion_drops_on_this_thread();
            let result = host_web::miso_engine_web_v1_render(handle, QUANTUM_FRAMES);
            run.drops_in_render += companion_drops_on_this_thread() - drops;
            run.render_allocations +=
                host_web::miso_engine_web_v1_render_allocation_count() - allocations;
            assert_eq!(result, RESULT_OK, "block {block}: the render");
            run.bits.push(
                native_staging::output_pcm(handle)
                    .expect("output")
                    .iter()
                    .map(|sample| sample.to_bits())
                    .collect(),
            );
            run.epochs
                .push(native_staging::active_plan_epoch(handle).expect("a plan"));
            run.clock.push(
                native_staging::status(handle)
                    .expect("a status")
                    .next_absolute_sample,
            );
            if republish && block == PUBLISH_BEFORE {
                // The first service call after the swap block reclaims the displaced plan.
                assert_eq!(
                    companion_drops_on_this_thread() - drops_before,
                    0,
                    "the swap block dropped nothing"
                );
                let control = host_web::miso_engine_web_v1_control_allocation_count();
                assert_eq!(host_web::miso_engine_web_v1_service(handle), RESULT_OK);
                run.control_calls_at_first_service =
                    host_web::miso_engine_web_v1_control_allocation_count() - control;
                run.drops_at_first_service = companion_drops_on_this_thread() - drops_before;
            }
        }
        assert_eq!(host_web::miso_engine_web_v1_service(handle), RESULT_OK);
        run.drops_in_total = companion_drops_on_this_thread() - drops_before;
        assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
        run
    })
    .join()
    .expect("the run completes")
}

/// Gate 2 (swap, clock, bits and reclaim) in its single-thread form, which is gate 4.
///
/// Test value: red if the browser still renders its plan directly (the epoch never moves), if the
/// clock restarts on adoption, if the successor renders other bits than the run without the
/// publication, if a render call allocates or frees (the displaced plan or its attachment dropping
/// on the render side), or if the service step never reclaims the displaced plan and its
/// attachment, or reclaims them where the control counter cannot see it. No other test swaps a
/// plan in the browser host: a boot alone never does.
#[test]
fn a_published_plan_is_adopted_at_a_block_boundary_and_reclaimed_by_the_service_step() {
    let _turn = TURN.lock().expect("the turn");
    let baseline = run(false);
    let swapped = run(true);

    assert_eq!(baseline.epochs, vec![0; BLOCKS], "no publication, no swap");
    let mut expected_epochs = vec![0; PUBLISH_BEFORE];
    expected_epochs.extend(vec![1; BLOCKS - PUBLISH_BEFORE]);
    assert_eq!(
        swapped.epochs, expected_epochs,
        "the block after the publication runs the successor"
    );
    assert_eq!(
        swapped.clock, baseline.clock,
        "the clock continues across the adoption"
    );
    assert_eq!(
        swapped.clock[BLOCKS - 1],
        (BLOCKS as u64) * u64::from(QUANTUM_FRAMES)
    );
    for (block, (after, before)) in swapped.bits.iter().zip(&baseline.bits).enumerate() {
        let first = after.iter().zip(before).position(|(a, b)| a != b);
        assert_eq!(
            first, None,
            "block {block}: a successor of the same audible model renders the same bits"
        );
    }

    // No render call allocated or freed, in either run.
    assert_eq!(baseline.render_allocations, 0);
    assert_eq!(
        swapped.render_allocations, 0,
        "a render call allocated or freed"
    );
    // Reclaim: render dropped nothing; the first service call after the swap block reclaims
    // exactly one plan, and its attachment drops there, on the calling thread, where the control
    // counter counts the frees.
    assert_eq!(swapped.drops_in_render, 0, "render dropped an attachment");
    assert_eq!(baseline.drops_in_render, 0);
    assert_eq!(
        swapped.drops_at_first_service, 1,
        "one displaced attachment dropped at the first service call"
    );
    assert!(
        swapped.control_calls_at_first_service > 0,
        "the service step's frees are control allocations"
    );
    assert_eq!(
        swapped.drops_in_total, 1,
        "no later step reclaims anything more"
    );
    assert_eq!(baseline.drops_in_total, 0, "nothing was displaced");
}

/// Gate 5: the watermark export copies #1314's record, field for field.
///
/// Test value: red if the export reads anything but the plan's watermark (the status words carry
/// no counters), if it reports a revision before render applied it, or if a wrong handle writes.
#[test]
fn the_watermark_export_copies_the_applied_revision_and_refuses_a_wrong_handle() {
    let _turn = TURN.lock().expect("the turn");
    std::thread::spawn(|| {
        let handle = boot();
        let model = parse_session_json(&document()).expect("the fixture parses");

        // Before any publication the plan applies the boot revision.
        assert_eq!(
            host_web::miso_engine_web_v1_watermark_read(handle),
            RESULT_OK
        );
        let boot_record = native_staging::watermark(handle).expect("a record");
        assert_eq!(boot_record.struct_size, WATERMARK_BYTES);
        assert_eq!(WATERMARK_BYTES, 96);
        assert_eq!(boot_record.reserved0, 0);
        assert_eq!(boot_record.reserved, [0; 5]);
        assert_eq!(boot_record.first_sample, 0);
        assert_eq!(boot_record.revision, model.revision);

        // A published revision is not applied until render has adopted it.
        let request = transaction_bytes(&add_muted_track(&model, "a-muted"), model.revision);
        let (result, response) =
            native_staging::control_command(handle, &request).expect("a live handle");
        assert_eq!(result, RESULT_OK, "the structural transaction publishes");
        assert_rebuild_applied(&response, model.revision + 1);
        assert_eq!(
            host_web::miso_engine_web_v1_watermark_read(handle),
            RESULT_OK
        );
        assert_eq!(
            native_staging::watermark(handle).expect("a record"),
            boot_record,
            "the watermark is never early"
        );
        assert_eq!(
            host_web::miso_engine_web_v1_render(handle, QUANTUM_FRAMES),
            RESULT_OK
        );
        assert_eq!(
            host_web::miso_engine_web_v1_watermark_read(handle),
            RESULT_OK
        );
        let applied = native_staging::watermark(handle).expect("a record");
        assert_eq!(
            applied.revision,
            model.revision + 1,
            "the committed revision is applied"
        );
        assert_eq!(applied.first_sample, 0, "the first block applied it");
        assert_ne!(applied, boot_record);

        // A wrong handle is refused and writes nothing.
        assert_eq!(
            host_web::miso_engine_web_v1_watermark_read(handle + 1),
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(
            native_staging::watermark(handle).expect("a record"),
            applied,
            "a refused read leaves the record"
        );
        assert_eq!(host_web::miso_engine_web_v1_service(handle), RESULT_OK);
        assert_eq!(
            host_web::miso_engine_web_v1_service(handle + 1),
            RESULT_INVALID_ARGUMENT
        );
        assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
    })
    .join()
    .expect("the watermark phase completes");
}

/// A rebuild adds a source whose ID has the longest length the grammar allows and whose channel
/// count exceeds every boot source's (#1562b review B1). Through the exports the worklet calls:
/// `source_id` copies the ID into staging (it trapped on a staging sized from the boot session),
/// the channel and frame queries answer the session map, and the source takes a six-plane chunk
/// and is heard (the staging held two planes, so the worklet's source rows failed and the source
/// could never be fed).
///
/// Test value: red if the browser's ID or PCM staging is sized from the boot session. The parity
/// row in `control_parity.rs` compares the bits against the C ABI through the host's safe API,
/// which bypasses the staging; only the exports exercise it.
#[test]
fn a_rebuild_adds_the_longest_id_with_more_channels_than_any_boot_source() {
    let _turn = TURN.lock().expect("the turn");
    std::thread::spawn(|| {
        let handle = boot();
        let model = parse_session_json(&document()).expect("the fixture parses");
        let name = format!("s{}", "x".repeat(session::STABLE_ID_MAXIMUM_BYTES - 1));
        let mut source = model.sources[0].clone();
        source.id = StableId::parse(&name).expect("source ID");
        source.channels = 6;
        let mut track = model.tracks[0].clone();
        track.id = StableId::parse("wide").expect("track ID");
        track.source_id = source.id.clone();
        track.left_source_channel = 4;
        track.right_source_channel = 5;
        let mut route = model.routes[0].clone();
        route.id = StableId::parse("wide-main").expect("route ID");
        let RouteSource::Track { track_id, .. } = &mut route.source else {
            panic!("a track route")
        };
        *track_id = track.id.clone();
        let edits = [
            SessionEdit::UpsertSource { source },
            SessionEdit::UpsertTrack { track },
            SessionEdit::UpsertRoute { route },
        ];
        let request = transaction_bytes(&edits, model.revision);
        let (result, response) =
            native_staging::control_command(handle, &request).expect("a live handle");
        assert_eq!(result, RESULT_OK, "the structural transaction publishes");
        assert_rebuild_applied(&response, model.revision + 1);
        assert_eq!(
            host_web::miso_engine_web_v1_render(handle, QUANTUM_FRAMES),
            RESULT_OK,
            "the swap block"
        );

        // The session map: canonical order puts the new ID after `fixture-source`.
        assert_eq!(host_web::miso_engine_web_v1_source_count(handle), 2);
        let length = host_web::miso_engine_web_v1_source_id(handle, 1);
        assert_eq!(
            length as usize,
            name.len(),
            "the ID is not truncated or refused"
        );
        assert_eq!(
            native_staging::staged_source_id(handle, length).as_deref(),
            Some(name.as_bytes())
        );
        assert_eq!(host_web::miso_engine_web_v1_source_channels(handle, 1), 6);

        // Feed it a six-plane chunk of its own generation and hear it.
        assert!(native_staging::source_id(handle, name.as_bytes()));
        assert_eq!(
            host_web::miso_engine_web_v1_source_seek(handle, length, 2, 0),
            RESULT_OK
        );
        assert!(native_staging::source_pcm(handle, 0.5));
        assert_eq!(
            host_web::miso_engine_web_v1_source_submit(handle, length, 2, 0, 6, QUANTUM_FRAMES, 0),
            RESULT_OK,
            "a six-channel chunk fits the staging"
        );
        assert_eq!(
            host_web::miso_engine_web_v1_render(handle, QUANTUM_FRAMES),
            RESULT_OK
        );
        let heard = native_staging::output_pcm(handle)
            .expect("output")
            .iter()
            .any(|sample| *sample != 0.0);
        assert!(heard, "the new source is heard");
        assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
    })
    .join()
    .expect("the run completes");
}
