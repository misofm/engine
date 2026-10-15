//! Issue #1290a gate 8: a structural transaction applied while the browser instance plays costs a
//! render call nothing. Gates 1 and 2 of the issue run as scripts through the module's exports, on
//! a fresh thread, as the worklet makes the calls: the control half applies between render calls
//! (the message handler's place, decision 17 "single instance first"), and a render call runs
//! inside its render-locked window, so the instance's own counters read what it allocated and freed.
//!
//! * the render-locked count does not move across any render call, the adoption block included;
//! * the displaced plan, and the tables the host retired into it, drop in the service step, on the
//!   control side, and never inside a render call;
//! * the apply itself is control work: it allocates, and the control count sees it.
//!
//! Decision 15 rules that host-web's native allocation-count gates live in an integration binary,
//! never in `src/tests.rs`, which registers its own allocator. The workspace has no
//! `bench_support::alloc`, so this binary registers the module's own counting allocator, as
//! `plan_exchange_threads.rs` does.

#![cfg(feature = "test-support")]

use std::alloc::System;
use std::sync::Mutex;

use controls::{ControlAddress, Owner, Section};
use host_web::{
    RESULT_OK, RenderLockedAllocator, WebBootOptions, companion_drops_on_this_thread,
    native_staging,
};
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    TypedCommandFrame,
};
use session::{
    MatrixOrPan, ParameterChannel, RouteSource, SessionModel, StableId, canonical_session_json,
    parse_session_json,
};

#[global_allocator]
static ALLOCATOR: RenderLockedAllocator<System> = RenderLockedAllocator(System);

/// The tests read process-wide counters, so they take turns.
static TURN: Mutex<()> = Mutex::new(());

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE_ID: &[u8] = b"fixture-source";
const QUANTUM_FRAMES: u32 = 128;
const BLOCKS: usize = 14;
/// The block before which the structural transaction runs.
const APPLY_BEFORE: usize = 6;
/// The block before which the live fader edit runs (gate 2).
const EDIT_BEFORE: usize = 5;
const FADER_DB: u32 = 5;
const FADER_RAMP: u32 = 400;
const ADDED: &str = "a-muted";

/// Nine stereo tracks over one source, both console sections and every insert rack empty, every
/// track's filters enabled at its own cutoffs and a non-centre pan. An effect carries no state
/// across a swap before #1280a, so a session that held one would not render the same bits.
fn model() -> SessionModel {
    let mut model = parse_session_json(FIXTURE).expect("the fixture parses");
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    for (index, track) in model.tracks.iter_mut().enumerate() {
        track.console.clear();
        track.inserts.effects.clear();
        let step = index as f32;
        for (side, channel) in [&mut track.builtins.left, &mut track.builtins.right]
            .into_iter()
            .enumerate()
        {
            channel.hpf_hz = 30.0 + 11.0 * step + 7.0 * side as f32;
            channel.lpf_hz = 9_000.0 - 450.0 * step - 600.0 * side as f32;
        }
        track.matrix_or_pan = MatrixOrPan::Pan {
            left: 0.9 - 0.05 * step,
            right: 0.4 + 0.05 * step,
            smoothing_samples: 16,
        };
    }
    model
}

/// A muted copy of the first track, whose ID sorts first, and its route to the output.
fn muted_track(model: &SessionModel) -> (session::Track, session::Route) {
    let mut track = model.tracks[0].clone();
    track.id = StableId::parse(ADDED).expect("track ID");
    track.fader.left_mute = true;
    track.fader.right_mute = true;
    let mut route = model.routes[0].clone();
    route.id = StableId::parse(&format!("{ADDED}-main")).expect("route ID");
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("a track route")
    };
    *track_id = track.id.clone();
    (track, route)
}

fn add_muted_track(model: &SessionModel) -> Vec<SessionEdit> {
    let (track, route) = muted_track(model);
    vec![
        SessionEdit::UpsertTrack { track },
        SessionEdit::UpsertRoute { route },
    ]
}

/// The committed model of the structural transaction: a fresh boot of it is the reference.
fn with_muted_track(model: &SessionModel) -> SessionModel {
    let (track, route) = muted_track(model);
    let mut next = model.clone();
    next.tracks.insert(0, track);
    next.routes.push(route);
    next.revision += 1;
    next
}

fn frame(edits: &[SessionEdit], request: u64, revision: u64) -> Vec<u8> {
    let codec = ProtocolCodec::default();
    let mut bytes = vec![0_u8; codec.limits().max_frame_bytes];
    let length = codec
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(request).expect("nonzero request"),
                expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                payload: CommandPayload::SessionTransactionApply(edits),
            },
            &mut bytes,
        )
        .expect("a transaction frame");
    bytes.truncate(length);
    bytes
}

fn applied(response: &[u8]) -> (EditPath, u64) {
    let mut fields = [0_u16; 64];
    let DecodedTypedResponseFrame::Success { header, payload } = ProtocolCodec::default()
        .decode_typed_response(response, &mut DecodeScratch::new(&mut fields))
        .expect("a transaction response")
    else {
        panic!("the transaction was refused")
    };
    let DecodedSuccessResponsePayload::SessionTransactionApplied(applied) = payload else {
        panic!("a transaction success")
    };
    (applied.edit_path, header.revision.0)
}

fn boot(model: &SessionModel) -> u32 {
    let options = WebBootOptions {
        live_control_meter_blocks: 2,
        live_control_observation_taps: 4,
        ..WebBootOptions::explicit_defaults()
    };
    let document = canonical_session_json(model).expect("canonical session");
    let length = native_staging::boot(options, document.as_bytes()).expect("the fixture stages");
    let handle = host_web::miso_engine_web_v1_boot(length);
    assert_ne!(handle, 0, "the fixture boots");
    handle
}

/// A deterministic block-dependent plane value, never zero.
fn sample(block: usize) -> f32 {
    0.05 + 0.02 * block as f32
}

/// Feed one quantum to the source, through the export.
fn feed(handle: u32, block: usize) {
    assert!(native_staging::source_id(handle, SOURCE_ID));
    assert!(native_staging::source_pcm(handle, sample(block)));
    let result = host_web::miso_engine_web_v1_source_submit(
        handle,
        SOURCE_ID.len() as u32,
        1,
        (block as u64) * u64::from(QUANTUM_FRAMES),
        2,
        QUANTUM_FRAMES,
        0,
    );
    assert_eq!(result, RESULT_OK, "block {block}: the source feed");
}

/// What a script left to compare.
struct Run {
    /// The output planes of every block, as bits.
    bits: Vec<Vec<u32>>,
    /// The epoch of the plan that rendered each block.
    epochs: Vec<u64>,
    /// Render-locked allocator calls over every render call: it must stay zero.
    render_allocations: u32,
    /// Attachments that dropped inside a render call, over the whole run.
    drops_in_render: u64,
    /// Attachments that dropped by the service call after the adoption block.
    drops_at_first_service: u64,
    /// Control allocator calls across the structural apply: the blocking rebuild, counted.
    control_calls_in_apply: u32,
    /// Control allocator calls across that service call.
    control_calls_in_service: u32,
    /// The instance's blocking-rebuild count at the end.
    blocking_rebuilds: u32,
    /// The watermark after the last block: its revision and first sample.
    watermark: (u64, u64),
}

/// What the script does.
#[derive(Clone, Copy, PartialEq)]
enum Script {
    /// Boot the committed model of the structural transaction, and apply nothing.
    FreshCommitted,
    /// Boot the model and apply the structural transaction before block `APPLY_BEFORE`.
    Structural,
    /// A live fader edit before block `EDIT_BEFORE`, then the structural transaction.
    EditThenStructural,
    /// The live fader edit alone.
    EditOnly,
}

fn run(script: Script) -> Run {
    std::thread::spawn(move || {
        let base = model();
        let handle = boot(&if script == Script::FreshCommitted {
            with_muted_track(&base)
        } else {
            base.clone()
        });
        let edited = base.tracks[3].id.as_str().to_owned();
        let mut run = Run {
            bits: Vec::new(),
            epochs: Vec::new(),
            render_allocations: 0,
            drops_in_render: 0,
            drops_at_first_service: 0,
            control_calls_in_apply: 0,
            control_calls_in_service: 0,
            blocking_rebuilds: 0,
            watermark: (0, 0),
        };
        let drops_before = companion_drops_on_this_thread();
        let mut request = 0;
        for block in 0..BLOCKS {
            feed(handle, block);
            if matches!(script, Script::EditThenStructural | Script::EditOnly)
                && block == EDIT_BEFORE
            {
                request += 1;
                let edit = [SessionEdit::SetControl {
                    address: ControlAddress {
                        owner: Owner::Strip(StableId::parse(&edited).expect("strip ID")),
                        section: Section::Builtins,
                        control: FADER_DB,
                        lane: ParameterChannel::Both,
                    },
                    value: -7.5,
                    ramp_samples: Some(FADER_RAMP),
                }];
                let revision = host_web::miso_engine_web_v1_control_revision(handle);
                let (result, response) =
                    native_staging::control_command(handle, &frame(&edit, request, revision))
                        .expect("a live handle");
                assert_eq!(result, RESULT_OK, "the live edit");
                assert_eq!(applied(&response).0, EditPath::Live);
            }
            if matches!(script, Script::Structural | Script::EditThenStructural)
                && block == APPLY_BEFORE
            {
                request += 1;
                let revision = host_web::miso_engine_web_v1_control_revision(handle);
                assert_eq!(native_staging::active_plan_epoch(handle), Some(0));
                let control = host_web::miso_engine_web_v1_control_allocation_count();
                let (result, response) = native_staging::control_command(
                    handle,
                    &frame(&add_muted_track(&base), request, revision),
                )
                .expect("a live handle");
                assert_eq!(result, RESULT_OK, "the structural transaction publishes");
                run.control_calls_in_apply =
                    host_web::miso_engine_web_v1_control_allocation_count() - control;
                assert_eq!(
                    applied(&response),
                    (EditPath::Rebuild, revision + 1),
                    "the response is the committed revision and the rebuild path"
                );
                assert_eq!(
                    native_staging::active_plan_epoch(handle),
                    Some(0),
                    "apply returns before render adopts"
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
            if matches!(script, Script::Structural | Script::EditThenStructural)
                && block == APPLY_BEFORE
            {
                // The first service call after the adoption block reclaims the displaced plan,
                // the tables the host retired into it with it.
                assert_eq!(companion_drops_on_this_thread() - drops_before, 0);
                let control = host_web::miso_engine_web_v1_control_allocation_count();
                assert_eq!(host_web::miso_engine_web_v1_service(handle), RESULT_OK);
                run.control_calls_in_service =
                    host_web::miso_engine_web_v1_control_allocation_count() - control;
                run.drops_at_first_service = companion_drops_on_this_thread() - drops_before;
            }
        }
        assert_eq!(
            host_web::miso_engine_web_v1_watermark_read(handle),
            RESULT_OK
        );
        let watermark = native_staging::watermark(handle).expect("a record");
        run.watermark = (watermark.revision, watermark.first_sample);
        run.blocking_rebuilds = native_staging::blocking_rebuilds(handle).expect("a handle");
        assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
        run
    })
    .join()
    .expect("the run completes")
}

fn first_difference(left: &[Vec<u32>], right: &[Vec<u32>]) -> Option<(usize, usize)> {
    left.iter()
        .zip(right)
        .enumerate()
        .find_map(|(block, (a, b))| {
            a.iter()
                .zip(b)
                .position(|(x, y)| x != y)
                .map(|sample| (block, sample))
        })
}

/// Gate 8, on gate 1's script (and the adoption of gate 1's successor).
///
/// Test value: red if the render half allocates or frees on adoption (the displaced plan, its
/// tables or the exchange dropping inside the render call), if the swap block's bits differ from
/// a fresh boot of the committed model, or if the apply's rebuild goes uncounted (the control
/// count would not move and `blockingRebuilds` would read zero).
#[test]
fn a_structural_apply_and_its_adoption_cost_no_render_call_an_allocation() {
    let _turn = TURN.lock().expect("the turn");
    let fresh = run(Script::FreshCommitted);
    let structural = run(Script::Structural);

    let mut expected_epochs = vec![0; APPLY_BEFORE];
    expected_epochs.extend(vec![1; BLOCKS - APPLY_BEFORE]);
    assert_eq!(structural.epochs, expected_epochs);
    assert_eq!(fresh.epochs, vec![0; BLOCKS]);
    assert_eq!(
        first_difference(&structural.bits, &fresh.bits),
        None,
        "every block equals a fresh boot of the committed model"
    );
    assert!(structural.bits[BLOCKS - 1].iter().any(|bits| *bits != 0));
    assert_eq!(
        structural.watermark,
        (
            model().revision + 1,
            (APPLY_BEFORE as u64) * u64::from(QUANTUM_FRAMES)
        ),
        "the watermark reports the adoption block"
    );

    assert_eq!(fresh.render_allocations, 0);
    assert_eq!(
        structural.render_allocations, 0,
        "a render call allocated or freed"
    );
    assert_eq!(
        structural.drops_in_render, 0,
        "render dropped an attachment"
    );
    assert_eq!(
        structural.drops_at_first_service, 1,
        "the displaced plan, its companions and the retired tables, dropped once in the service step"
    );
    assert!(
        structural.control_calls_in_apply > 0,
        "the apply is control work, counted"
    );
    assert!(
        structural.control_calls_in_service > 0,
        "the reclaim's frees are control allocations"
    );
    assert_eq!(structural.blocking_rebuilds, 1);
    assert_eq!(fresh.blocking_rebuilds, 0);
}

/// Gate 8, on gate 2's script.
///
/// Test value: red if a ramp in flight at the adoption block makes the render call allocate, or
/// if the swap disturbs the run with the live edit alone.
#[test]
fn a_live_edit_in_flight_at_the_adoption_block_costs_no_render_call_an_allocation() {
    let _turn = TURN.lock().expect("the turn");
    let edit_only = run(Script::EditOnly);
    let swapped = run(Script::EditThenStructural);
    assert_eq!(
        first_difference(&swapped.bits, &edit_only.bits),
        None,
        "the swap disturbs nothing, the ramp in flight included"
    );
    assert_eq!(swapped.render_allocations, 0);
    assert_eq!(swapped.drops_in_render, 0);
    assert_eq!(swapped.drops_at_first_service, 1);
    assert_eq!(swapped.blocking_rebuilds, 1);
}

/// What a metered script published.
struct Metered {
    /// Every frame the polls published, in order: the header and the frame's words.
    frames: Vec<(host_web::WebMeterHeader, Vec<f32>)>,
    /// Render-locked allocator calls over every render call and every meter poll.
    render_allocations: u32,
    /// The frames the polls after the block `ADOPTION_WITH_WINDOW` published.
    published_at_adoption: usize,
}

/// Windows close after the odd blocks (a two-block period), so a transaction before block 5 is
/// adopted by the block that closes the window opening at sample 512.
const ADOPTION_WITH_WINDOW: usize = 5;

fn run_metered(structural: bool) -> Metered {
    std::thread::spawn(move || {
        let base = model();
        let handle = boot(&if structural {
            base.clone()
        } else {
            with_muted_track(&base)
        });
        assert_eq!(
            host_web::miso_engine_web_v1_meter_lease(handle, 1),
            RESULT_OK
        );
        let mut run = Metered {
            frames: Vec::new(),
            render_allocations: 0,
            published_at_adoption: 0,
        };
        for block in 0..BLOCKS {
            feed(handle, block);
            if structural && block == ADOPTION_WITH_WINDOW {
                let revision = host_web::miso_engine_web_v1_control_revision(handle);
                let (result, _) = native_staging::control_command(
                    handle,
                    &frame(&add_muted_track(&base), 1, revision),
                )
                .expect("a live handle");
                assert_eq!(result, RESULT_OK, "the structural transaction publishes");
            }
            let allocations = host_web::miso_engine_web_v1_render_allocation_count();
            assert_eq!(
                host_web::miso_engine_web_v1_render(handle, QUANTUM_FRAMES),
                RESULT_OK,
                "block {block}: the render"
            );
            let windows = host_web::miso_engine_web_v1_meter_poll(handle);
            run.render_allocations +=
                host_web::miso_engine_web_v1_render_allocation_count() - allocations;
            if windows > 0 {
                run.frames
                    .push(native_staging::meter_frame(handle).expect("a handle"));
                if block == ADOPTION_WITH_WINDOW {
                    run.published_at_adoption += 1;
                }
            }
            if structural && block == ADOPTION_WITH_WINDOW {
                assert_eq!(host_web::miso_engine_web_v1_service(handle), RESULT_OK);
            }
        }
        assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
        run
    })
    .join()
    .expect("the run completes")
}

/// #1290d1 gate: a strip-count change publishes a correctly shaped frame on the first block after
/// adoption, with zero render allocations. The frame travels with the plan's tables, so the
/// adoption block exchanges a pointer and the poll that follows writes into the successor's frame.
///
/// Test value: red if the swap block or the poll after it allocates or frees (a frame cut or
/// grown in render), if the first poll after adoption publishes nothing or a frame of the
/// predecessor's size, or if the window is not what a fresh boot of the committed model publishes
/// for it.
#[test]
fn a_strip_count_change_publishes_the_successors_frame_on_the_first_block_after_adoption() {
    let _turn = TURN.lock().expect("the turn");
    let fresh = run_metered(false);
    let swapped = run_metered(true);

    assert_eq!(fresh.render_allocations, 0);
    assert_eq!(
        swapped.render_allocations, 0,
        "a render call or a poll allocated or freed"
    );
    assert_eq!(
        swapped.published_at_adoption, 1,
        "the poll after the adoption block publishes"
    );
    let sizes = |run: &Metered| {
        run.frames
            .iter()
            .map(|(header, frame)| (header.track_count, frame.len()))
            .collect::<Vec<_>>()
    };
    let published = sizes(&swapped);
    assert_eq!(
        published.first(),
        Some(&(9, 3 * 9 + 3)),
        "the boot plan's frames"
    );
    let adopted = published
        .iter()
        .position(|shape| *shape == (10, 3 * 10 + 3))
        .expect("the successor's frames follow");
    assert!(
        published[adopted..]
            .iter()
            .all(|shape| *shape == (10, 3 * 10 + 3)),
        "every frame after the adoption block is the successor's"
    );
    let (adoption_header, adoption_frame) = &swapped.frames[adopted];
    assert_eq!(
        adoption_header.first_sample,
        (ADOPTION_WITH_WINDOW as u64 - 1) * u64::from(QUANTUM_FRAMES)
    );
    assert_eq!(
        adoption_header.sequence,
        swapped.frames[adopted - 1].0.sequence + 1,
        "the stream continues"
    );
    assert_eq!(adoption_header.reserved[1] >> 32, 0, "nothing counted lost");
    for (header, frame) in &swapped.frames[adopted..] {
        let (_, reference) = fresh
            .frames
            .iter()
            .find(|(candidate, _)| candidate.first_sample == header.first_sample)
            .expect("the fresh boot published the same window");
        assert_eq!(
            frame.iter().map(|word| word.to_bits()).collect::<Vec<_>>(),
            reference
                .iter()
                .map(|word| word.to_bits())
                .collect::<Vec<_>>(),
            "the window opening at {} equals a fresh boot of the committed model",
            header.first_sample
        );
    }
    assert!(adoption_frame.iter().any(|word| *word != 0.0));
}
