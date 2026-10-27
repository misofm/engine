//! Issue #1332 S5 (with #1562a gate 5): the control half and the render half of one running host
//! on two threads, as the Worker and the AudioWorklet will hold them. The control thread commits
//! structural and live edits, feeds the source, drains the event lanes and runs the service step
//! while the render thread renders, adopts each published plan at a block boundary and hands the
//! retired tables back, so every control-to-render signal of the split host crosses a thread: the
//! plan exchange, the source rings, each plan's own words, the successor record, the lifecycle
//! word and the hand-back ring.
//!
//! The render thread drives the released render half ([`host_web::RenderHost`]) inside the
//! render-locked window, as the worklet's render export will (S6 wires the export), so the
//! module's counting allocator counts every allocator call a render makes: the count must stay
//! exactly zero. The control thread's allocations are control allocations and do not count.
//!
//! Decision 15 rules that host-web's native allocation-count gates live in an integration binary,
//! never in `src/tests.rs`, which registers its own allocator. The workspace has no
//! `bench_support::alloc`, so this binary registers the module's own counting allocator, as
//! `plan_exchange_threads.rs` does. The two tests read its process-wide counters in turn.

#![cfg(feature = "test-support")]

use std::alloc::System;
use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

use controls::{ControlAddress, Owner, Section};
use host_web::{
    AudioWorkletEngineHost, RESULT_BACKPRESSURE, RESULT_OK, RenderLockedAllocator, WebBootOptions,
    native_staging, render_locked_for_test,
};
use protocol::{
    CommandPayload, DecodeScratch, DecodedSuccessResponsePayload, DecodedTypedResponseFrame,
    EditPath, ExpectedRevision, ProtocolCodec, RequestId, SessionEdit, SessionRevision,
    TypedCommandFrame,
};
use session::{
    ParameterChannel, RouteSource, SessionModel, StableId, canonical_session_json,
    parse_session_json,
};

mod common;

#[global_allocator]
static ALLOCATOR: RenderLockedAllocator<System> = RenderLockedAllocator(System);

/// The tests read process-wide counters, so they take turns.
static TURN: Mutex<()> = Mutex::new(());

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE_ID: &[u8] = b"fixture-source";
const QUANTUM_FRAMES: u32 = 128;
/// The blocks each run renders: the fixture's source holds 375.
const BLOCKS: u64 = 360;
/// How far ahead of render the control thread may feed, in blocks: the source ring's length, so
/// render runs ahead freely while the control thread commits.
const RING_BLOCKS: u64 = 256;
/// How many blocks render may run past the block of the action the control thread is about to
/// run: the window in which render adopts plans while the control thread commits.
const LEAD: u64 = 12;
const FADER_DB: u32 = 5;
/// The `master` monitor row of a strip (#1683).
const MASTER_ROW: u32 = 4;
/// The strip the run designates the master: the last in canonical order, before and after every
/// added track (`a-muted-*` sorts first), so its index in each plan's strip order is that plan's
/// track count less one.
const MASTER: &str = "eq8";

/// What the control thread does before the block `at` is rendered, in the script's order.
#[derive(Clone, Copy)]
enum Action {
    /// Add the muted track `a-muted-{0}` and its route: a rebuild whose plan adds exact `+0.0` to
    /// the output sum, so the output bits do not depend on the block that adopts it.
    AddMutedTrack(u64),
    /// Set the fader of the muted track `a-muted-{0}`: a live edit that moves no output bit.
    MutedFader(u64, bool),
}

/// The script both runs play: rebuilds, some of them back to back (the second supersedes the
/// first unless render has taken it), and live edits between them.
fn script() -> Vec<(u64, Action)> {
    let mut script = Vec::new();
    let mut added = 0;
    for at in (16..BLOCKS - 40).step_by(24) {
        script.push((at, Action::AddMutedTrack(added)));
        added += 1;
        if at % 48 == 16 {
            script.push((at, Action::AddMutedTrack(added)));
            added += 1;
        }
        for live in 1..4 {
            script.push((at + 5 * live, Action::MutedFader(added - 1, live % 2 == 0)));
        }
    }
    script
}

/// Nine stereo tracks over one source, with both console sections empty, no insert and the input
/// filters off, as `plan_exchange_threads.rs` builds it: a console slot, an insert and an input
/// filter carry no state across a plan swap yet (#1281, #1282, #1277 D6), so only this session
/// renders the same bits whichever block adopts a plan.
fn model() -> SessionModel {
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
    model
}

fn muted_id(index: u64) -> String {
    format!("a-muted-{index}")
}

/// A muted copy of the first track and its route to the output.
fn add_muted_track(model: &SessionModel, index: u64) -> [SessionEdit; 2] {
    let id = muted_id(index);
    let mut track = model.tracks[0].clone();
    track.id = StableId::parse(&id).expect("track ID");
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

fn set_control(owner: &str, section: Section, control: u32, value: f32) -> SessionEdit {
    SessionEdit::SetControl {
        expected_effect: None,
        address: ControlAddress {
            owner: Owner::Strip(StableId::parse(owner).expect("strip ID")),
            section,
            control,
            lane: ParameterChannel::Both,
        },
        value,
        ramp_samples: None,
    }
}

fn transaction(edits: &[SessionEdit], request: u64, revision: u64) -> Vec<u8> {
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

/// What the control thread holds of the session: the host with its control half, the request and
/// revision counters, and the words each published plan must show, by plan epoch.
struct Control {
    host: AudioWorkletEngineHost,
    model: SessionModel,
    request: u64,
    revision: u64,
    added: u64,
    /// `(structure epoch, track count)` of each plan the control half published, by epoch.
    expected: BTreeMap<u64, (u64, u32)>,
}

impl Control {
    fn boot() -> Self {
        let model = model();
        let document = canonical_session_json(&model).expect("canonical");
        let options = WebBootOptions {
            require_quantum_frames: QUANTUM_FRAMES,
            source_ring_frames: QUANTUM_FRAMES * RING_BLOCKS as u32,
            ..WebBootOptions::explicit_defaults()
        };
        let host = AudioWorkletEngineHost::boot(document.as_bytes(), options)
            .unwrap_or_else(|failure| panic!("boot: {:?}", failure.diagnostic()));
        let revision = model.revision;
        let mut control = Self {
            host,
            model,
            request: 0,
            revision,
            added: 0,
            expected: BTreeMap::new(),
        };
        // A monitor write, which spends no revision.
        let (path, _) = control.command(&[set_control(MASTER, Section::Own, MASTER_ROW, 1.0)]);
        assert_eq!(path, EditPath::Monitor);
        control.expect_newest();
        control
    }

    /// Run one transaction through the production control entry; the edit path it took.
    fn command(&mut self, edits: &[SessionEdit]) -> (EditPath, u64) {
        self.request += 1;
        let frame = transaction(edits, self.request, self.revision);
        self.host
            .control_request_mut(frame.len() as u32)
            .expect("the control staging")[..frame.len()]
            .copy_from_slice(&frame);
        assert_eq!(self.host.control_command(frame.len() as u32), RESULT_OK);
        let mut fields = [0_u16; 64];
        let DecodedTypedResponseFrame::Success { header, payload } = ProtocolCodec::default()
            .decode_typed_response(
                self.host.control_response(),
                &mut DecodeScratch::new(&mut fields),
            )
            .expect("a transaction response")
        else {
            panic!("transaction {} was refused", self.request)
        };
        let DecodedSuccessResponsePayload::SessionTransactionApplied(applied) = payload else {
            panic!("a transaction success")
        };
        self.revision = header.revision.0;
        (applied.edit_path, header.revision.0)
    }

    /// Record the words the newest plan must show: the structure epoch the commit left, and its
    /// track count, which is the master's index plus one.
    fn expect_newest(&mut self) {
        let (epoch, structure) = self.host.newest_plan_words().expect("a control half");
        let tracks = (self.model.tracks.len() as u64 + self.added) as u32;
        self.expected.insert(epoch, (structure, tracks));
    }

    fn act(&mut self, action: Action) {
        match action {
            Action::AddMutedTrack(index) => {
                assert_eq!(index, self.added);
                let edits = add_muted_track(&self.model, index);
                let (path, _) = self.command(&edits);
                assert_eq!(path, EditPath::Rebuild);
                self.added += 1;
                self.expect_newest();
            }
            Action::MutedFader(index, high) => {
                let db = if high { -3.0 } else { -6.0 };
                let edits = [set_control(
                    &muted_id(index),
                    Section::Builtins,
                    FADER_DB,
                    db,
                )];
                let (path, _) = self.command(&edits);
                assert_eq!(path, EditPath::Live);
            }
        }
    }

    /// Feed block `block`; `false` when the ring has no room for it yet.
    fn feed(&mut self, block: u64) -> bool {
        let plane = |lane: u64| {
            (0..u64::from(QUANTUM_FRAMES))
                .map(|frame| {
                    let at = block * u64::from(QUANTUM_FRAMES) + frame;
                    ((at * (3 + lane)) % 211) as f32 / 211.0 * 0.25 + 0.01
                })
                .collect::<Vec<f32>>()
        };
        let planes = [plane(0), plane(1)];
        let views: [&[f32]; 2] = [&planes[0], &planes[1]];
        let result = self.host.submit_source(
            SOURCE_ID,
            1,
            block * u64::from(QUANTUM_FRAMES),
            48_000,
            &views,
            QUANTUM_FRAMES,
            false,
        );
        assert!(
            result == RESULT_OK || result == RESULT_BACKPRESSURE,
            "block {block}: the source feed returned {result}"
        );
        result == RESULT_OK
    }

    /// The control half's side work: both event lanes drained, so neither fills, and one service
    /// step, which reclaims retired plans and the tables render handed back.
    fn service(&mut self) {
        for lane in [host_web::EVENT_LANE_RELIABLE, host_web::EVENT_LANE_LOSSY] {
            loop {
                assert_eq!(self.host.control_event(lane), RESULT_OK);
                if self.host.control_response().is_empty() {
                    break;
                }
            }
        }
        assert_eq!(self.host.service(), RESULT_OK);
    }
}

fn bits(output: Option<&[f32]>) -> Vec<u32> {
    output
        .expect("output")
        .iter()
        .map(|sample| sample.to_bits())
        .collect()
}

/// The script on one thread, the single instance's form: the reference bits.
fn single_thread() -> Vec<Vec<u32>> {
    std::thread::spawn(|| {
        let mut control = Control::boot();
        let script = script();
        let mut next = 0;
        let mut blocks = Vec::new();
        for block in 0..BLOCKS {
            while next < script.len() && script[next].0 <= block {
                control.act(script[next].1);
                next += 1;
            }
            assert!(control.feed(block));
            assert_eq!(
                render_locked_for_test(|| control.host.render_next()),
                RESULT_OK
            );
            blocks.push(bits(control.host.output_pcm()));
            control.service();
        }
        assert_eq!(control.host.dispose(), RESULT_OK);
        blocks
    })
    .join()
    .expect("the single-thread run completes")
}

/// What one block of the two-thread run showed: the epoch of the plan that rendered it, the
/// header's track count, master and structure epoch, and the output bits.
struct Block {
    epoch: u64,
    tracks: u32,
    master_plus_one: u32,
    structure_epoch: u64,
    bits: Vec<u32>,
}

/// What the two-thread run left.
struct TwoThreads {
    blocks: Vec<Block>,
    /// `(structure epoch, track count)` of each published plan, by epoch.
    expected: BTreeMap<u64, (u64, u32)>,
    /// Allocator calls inside the render thread's render calls.
    render_allocations: u32,
}

/// The script on two threads: the control thread commits, feeds and services while the render
/// thread renders as fast as the fed PCM allows.
fn two_threads() -> TwoThreads {
    let mut control = Control::boot();
    let mut render = control.host.release_render().expect("the render half");
    let fed = AtomicU64::new(0);
    let rendered = AtomicU64::new(0);
    let (blocks, render_allocations) = std::thread::scope(|scope| {
        let renderer = scope.spawn(|| {
            let mut blocks = Vec::with_capacity(BLOCKS as usize);
            let mut render_allocations = 0;
            for block in 0..BLOCKS {
                while fed.load(Ordering::Acquire) <= block {
                    std::hint::spin_loop();
                }
                let before = host_web::miso_engine_web_v1_render_allocation_count();
                let result = render_locked_for_test(|| render.render_next());
                render_allocations +=
                    host_web::miso_engine_web_v1_render_allocation_count() - before;
                assert_eq!(result, RESULT_OK, "block {block}: the render");
                let header = *render.meter_header();
                blocks.push(Block {
                    epoch: render.plan_epoch(),
                    tracks: header.track_count,
                    master_plus_one: header.master_track_plus_one,
                    structure_epoch: header.structure_epoch,
                    bits: bits(render.output_pcm()),
                });
                rendered.store(block + 1, Ordering::Release);
            }
            (blocks, render_allocations)
        });
        let script = script();
        let mut next = 0;
        let mut feed = 0;
        loop {
            let done = rendered.load(Ordering::Acquire);
            // Render may run up to `LEAD` blocks past the next action's block while the action
            // commits, and no further, so the script runs whole whatever the threads' speeds.
            let limit = script
                .get(next)
                .map_or(BLOCKS, |&(at, _)| (at + LEAD).min(BLOCKS));
            while feed < limit && feed < done + RING_BLOCKS && control.feed(feed) {
                feed += 1;
                fed.store(feed, Ordering::Release);
            }
            if next < script.len() && script[next].0 <= done {
                control.act(script[next].1);
                next += 1;
            }
            control.service();
            if done == BLOCKS {
                break;
            }
        }
        assert_eq!(next, script.len(), "the whole script ran");
        renderer.join().expect("the render thread completes")
    });
    assert!(
        control.host.adopt_render(render).is_ok(),
        "the host takes its render half back"
    );
    assert_eq!(control.host.dispose(), RESULT_OK);
    TwoThreads {
        blocks,
        expected: control.expected,
        render_allocations,
    }
}

/// #1332 S5: the control half commits structural and live edits on one thread while the render
/// half renders and adopts on another.
///
/// Test value: red if any signal of the split host reaches render through memory render frees,
/// allocates or locks (the render-locked count moves), in particular if render drops the tables
/// a swap retires instead of handing them back; red if a plan adopted on the render thread shows
/// any words but its own (no master and structure epoch `0`, or another plan's), as it does when
/// the plan is published before its words are stamped and render adopts it before the control
/// call returns; and red if a cross-thread handoff loses or reorders PCM (the bits differ from
/// the single-thread run of the same script).
#[test]
fn control_on_one_thread_and_render_on_another_never_allocate_in_render_and_match_one_thread() {
    let _turn = TURN
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let reference = single_thread();
    let run = two_threads();
    assert_eq!(
        run.render_allocations, 0,
        "a render call allocated or freed"
    );
    for (index, block) in run.blocks.iter().enumerate() {
        let &(structure, tracks) = run
            .expected
            .get(&block.epoch)
            .unwrap_or_else(|| panic!("block {index}: plan {} was never published", block.epoch));
        assert_eq!(
            (block.tracks, block.master_plus_one, block.structure_epoch),
            (tracks, tracks, structure),
            "block {index}, plan {}: the header shows another plan's words",
            block.epoch
        );
    }
    assert!(
        run.blocks.iter().any(|block| block.epoch > 0),
        "render adopted the published plans"
    );
    assert_eq!(run.blocks.len(), reference.len());
    for (index, (block, reference)) in run.blocks.iter().zip(&reference).enumerate() {
        assert_eq!(
            &block.bits, reference,
            "block {index}: the two-thread run's bits differ from the single-thread run's"
        );
    }
}

/// One live edit of each owner kind, by `kind % 5`: a strip fader, an insert parameter, a console
/// slot parameter, a send's gain and a VCA offset, over the parity session of `common`.
fn owner_edit(kind: u64, high: bool) -> SessionEdit {
    let id = |text: &str| StableId::parse(text).expect("stable ID");
    let (owner, section, control, lane, low, up) = match kind % 5 {
        0 => (
            Owner::Strip(id("t1")),
            Section::Builtins,
            common::FADER_DB,
            ParameterChannel::Both,
            -6.0,
            -3.0,
        ),
        1 => (
            Owner::Strip(id("t1")),
            Section::Inserts(id("comp")),
            1,
            ParameterChannel::Left,
            -30.0,
            -24.0,
        ),
        2 => (
            Owner::Strip(id("t1")),
            Section::Console(id(common::PRE_SLOT)),
            1,
            ParameterChannel::Left,
            -30.0,
            -24.0,
        ),
        3 => (
            Owner::Route(id("t1-b")),
            Section::Own,
            common::ROUTE_GAIN_DB,
            ParameterChannel::Both,
            -6.0,
            -3.0,
        ),
        _ => (
            Owner::Vca(id("v")),
            Section::Own,
            common::VCA_OFFSET_DB,
            ParameterChannel::Both,
            -3.0,
            -1.0,
        ),
    };
    SessionEdit::SetControl {
        expected_effect: None,
        address: ControlAddress {
            owner,
            section,
            control,
            lane,
        },
        value: if high { up } else { low },
        ramp_samples: None,
    }
}

fn committed(response: &[u8]) -> (EditPath, u64) {
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

/// 640 live `SET_CONTROL` transactions between renders, one owner kind at a time (a strip, an
/// insert, a console slot, a send, a VCA), on the parity session, in the single instance's form
/// (#1562a gate 5): the control frames allocate in the control count, and no render call
/// allocates or frees.
///
/// Test value: red if a live edit of an insert, a console slot, a send or a VCA does any control
/// work inside the next render call (the plan's lanes, a catalog row or a retired table dropped on
/// the render side), which the two-thread test does not reach: its session has no insert,
/// console slot, send or VCA, none of which carries its state across a plan swap yet.
#[test]
fn live_edits_of_every_owner_kind_between_renders_never_allocate_in_a_render_call() {
    const SOURCES: [&str; 3] = ["src-t1", "src-t2", "src-t3"];
    const BLOCKS: u64 = 640;
    let _turn = TURN
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    std::thread::spawn(|| {
        let model = common::model_at(48_000);
        let document = canonical_session_json(&model).expect("canonical");
        let options = WebBootOptions {
            require_sample_rate_hz: 48_000,
            require_quantum_frames: QUANTUM_FRAMES,
            source_ring_frames: QUANTUM_FRAMES * 2,
            ..WebBootOptions::explicit_defaults()
        };
        let length =
            native_staging::boot(options, document.as_bytes()).expect("the session stages");
        let handle = host_web::miso_engine_web_v1_boot(length);
        assert_ne!(handle, 0, "the parity session boots");
        let mut revision = model.revision;
        let mut render_allocations = 0;
        let mut control_calls = 0;
        for block in 0..BLOCKS {
            let control = host_web::miso_engine_web_v1_control_allocation_count();
            let edits = [owner_edit(block, (block / 5) % 2 == 1)];
            let codec = ProtocolCodec::default();
            let mut bytes = vec![0_u8; 4_096];
            let length = codec
                .encode_command_frame_into(
                    &TypedCommandFrame {
                        request_id: RequestId::new(block + 1).expect("nonzero request"),
                        expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                        payload: CommandPayload::SessionTransactionApply(&edits),
                    },
                    &mut bytes,
                )
                .expect("a transaction frame");
            let (result, response) =
                native_staging::control_command(handle, &bytes[..length]).expect("a live handle");
            assert_eq!(result, RESULT_OK, "block {block}: the control call");
            let (path, next) = committed(&response);
            assert_eq!(path, EditPath::Live, "block {block}: the edit is live");
            revision = next;
            for lane in [host_web::EVENT_LANE_RELIABLE, host_web::EVENT_LANE_LOSSY] {
                loop {
                    let (result, event) = native_staging::control_event(handle, lane);
                    assert_eq!(result, RESULT_OK, "block {block}: the event drain");
                    if event.is_empty() {
                        break;
                    }
                }
            }
            control_calls += host_web::miso_engine_web_v1_control_allocation_count() - control;
            for source in SOURCES {
                assert!(native_staging::source_id(handle, source.as_bytes()));
                assert!(native_staging::source_pcm(handle, 0.1));
                let fed = host_web::miso_engine_web_v1_source_submit(
                    handle,
                    source.len() as u32,
                    1,
                    block * u64::from(QUANTUM_FRAMES),
                    2,
                    QUANTUM_FRAMES,
                    0,
                );
                // The sources hold 72 blocks; a later feed is refused at the end of the region.
                if block < 64 {
                    assert_eq!(fed, RESULT_OK, "block {block}: the feed of {source}");
                }
            }
            let before = host_web::miso_engine_web_v1_render_allocation_count();
            let rendered = host_web::miso_engine_web_v1_render(handle, QUANTUM_FRAMES);
            render_allocations += host_web::miso_engine_web_v1_render_allocation_count() - before;
            assert_eq!(rendered, RESULT_OK, "block {block}: the render");
        }
        assert_eq!(render_allocations, 0, "a render call allocated or freed");
        assert!(
            control_calls > 0,
            "the control frames' allocations are counted"
        );
        assert_eq!(host_web::miso_engine_web_v1_dispose(handle), RESULT_OK);
    })
    .join()
    .expect("the run completes");
}
