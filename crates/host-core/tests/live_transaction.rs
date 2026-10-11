//! Issue #1312 gate 8: one live transaction takes effect in one block, and the watermark names it.
//!
//! Two strips, `A` and `B`, render through the exchange (`RealtimePlanOwner`). One transaction
//! mutes `A` and unmutes `B`: both are written into their cells, stamped with a revision `r` from
//! the plan's gate handle. While `r` is unpublished, the next block applies neither change and the
//! watermark stays below `r`; once `r` is published, the block after applies both, from its first
//! sample, and the watermark reads `(r, that block's first sample)`.

use core::num::NonZeroUsize;

use builtins::BuiltinLaneSelector;
use builtins_compiler::TrackFaderRecord;
use dsp_reference::randomized::Draw;
use engine::realtime::{
    PlanExchangeConfig, PlanarBufferMut, RealtimePlanOwner, RenderIo, RenderTime,
    plan_exchange_at_revision,
};
use host_core::{
    HostLiveControlRequest, HostLiveLanes, HostPrepareCaps, HostShapePolicy, LiveValuesSuperseded,
    PreparedHost, SourceSubmission, compile_host_session, prepare_host_runtime,
    prepare_host_runtime_with_live_lanes,
};
use session::{Console, SessionModel, canonical_session_json, parse_session_json};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE: &str = "fixture-source";
/// The strip the transaction mutes.
const A: usize = 2;
/// The strip the transaction unmutes; it starts muted.
const B: usize = 5;

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 4_096,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_submixes: 100,
        maximum_vcas: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 1_000_000_000,
        maximum_source_total_bytes: 100_000_000,
        maximum_source_overhead_bytes: 100_000_000,
        maximum_effect_state_bytes: 1_000_000_000,
        maximum_effect_scratch_bytes: 1_000_000_000,
        maximum_builtin_retained_bytes: 1_000_000_000,
        maximum_named_allocation_bytes: 1_000_000_000,
        maximum_meter_streams: 64,
        maximum_meter_items: 1 << 16,
        maximum_meter_bytes: 1 << 24,
    }
}

/// The nine-track fixture with an empty console, no inserts and both input filters off, so every
/// strip is stateless downstream of its fader and a mute is a step at its first sample. `A` is
/// open, `B` muted on both lanes; `muted` flips them.
fn model(a_muted: bool, b_muted: bool) -> SessionModel {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    model.console = Console {
        pre_insert: Vec::new(),
        post_insert: Vec::new(),
    };
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
        for lane in [&mut track.builtins.left, &mut track.builtins.right] {
            lane.hpf_hz = 0.0;
            lane.lpf_hz = 0.0;
        }
    }
    // `B` is quieter than `A`, so swapping their mutes changes the sum.
    model.tracks[B].fader.left_db = -9.0;
    model.tracks[B].fader.right_db = -9.0;
    for (index, muted) in [(A, a_muted), (B, b_muted)] {
        model.tracks[index].fader.left_mute = muted;
        model.tracks[index].fader.right_mute = muted;
    }
    model
}

fn compile(model: &SessionModel) -> session::CompiledSession {
    let document = canonical_session_json(model).expect("canonical");
    compile_host_session(&document, &caps()).unwrap_or_else(|failure| {
        panic!("compile: {}", String::from_utf8_lossy(failure.as_bytes()))
    })
}

fn feed(frames: usize) -> [Vec<f32>; 2] {
    let mut draw = Draw::new(0x1312);
    let mut plane = || -> Vec<f32> { (0..frames).map(|_| draw.noise(0.5)).collect() };
    [plane(), plane()]
}

fn submit(host: &mut PreparedHost, planes: &[Vec<f32>; 2], block: usize) {
    let quantum = host.report.quantum_frames as usize;
    let rate = host.report.sample_rate_hz;
    let range = block * quantum..(block + 1) * quantum;
    host.sources
        .submit(
            SOURCE.as_bytes(),
            SourceSubmission {
                generation: 1,
                start_frame: range.start as u64,
                sample_rate_hz: rate,
                planes: &[&planes[0][range.clone()], &planes[1][range]],
                frames: quantum as u32,
                end_of_region: false,
            },
        )
        .expect("source block");
}

fn output_bits(samples: &[f32]) -> Vec<u32> {
    samples.iter().map(|sample| sample.to_bits()).collect()
}

/// The bits of block `block` of a plain plan of `model`, fed `planes` from block 0.
fn reference_block(model: &SessionModel, planes: &[Vec<f32>; 2], block: usize) -> Vec<u32> {
    let mut host = prepare_host_runtime(&compile(model), &caps()).unwrap_or_else(|failure| {
        panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
    });
    let quantum = host.report.quantum_frames as usize;
    let mut last = Vec::new();
    for current in 0..=block {
        submit(&mut host, planes, current);
        let mut samples = vec![0.0_f32; quantum * 2];
        let output = PlanarBufferMut::try_new(&mut samples, 2, quantum, quantum).expect("planes");
        host.plan
            .render(
                RenderIo { output },
                RenderTime {
                    absolute_sample: (current * quantum) as u64,
                },
            )
            .expect("render");
        last = output_bits(&samples);
    }
    last
}

fn render(owner: &mut RealtimePlanOwner, quantum: usize, block: usize) -> Vec<u32> {
    let mut samples = vec![0.0_f32; quantum * 2];
    let output = PlanarBufferMut::try_new(&mut samples, 2, quantum, quantum).expect("planes");
    owner
        .render(
            RenderIo { output },
            RenderTime {
                absolute_sample: (block * quantum) as u64,
            },
        )
        .expect("render");
    output_bits(&samples)
}

/// Test value: a drain that applies a value above the block's snapshot, so a transaction lands
/// across two blocks (or lands in a block before its revision is published), or a watermark that
/// names a block before the one that applied the revision.
///
/// Red mutation: the strip drains read with `LiveSnapshot::ALL`; the first block then applies both
/// unpublished changes.
#[test]
fn one_transaction_is_applied_whole_in_the_block_its_revision_is_published_for() {
    let initial = model(false, true);
    let target = model(true, false);
    let planes = feed(128 * 64);

    let (mut host, mut handles) = prepare_host_runtime_with_live_lanes(
        &compile(&initial),
        &caps(),
        &HostLiveControlRequest {
            control_queue_depth: Some(NonZeroUsize::new(16).expect("depth")),
            ..HostLiveControlRequest::default()
        },
        HostLiveLanes::FADER_AND_MATRIX,
        None,
        &LiveValuesSuperseded::default(),
    )
    .unwrap_or_else(|failure| panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes())));
    let quantum = host.report.quantum_frames as usize;
    let mut gate = host
        .plan
        .take_revision_gate()
        .expect("the plan's gate handle");
    let revision = 7;
    let initial_plan = host.plan;
    let mut sources = host.sources;
    let (publisher, mut owner, _retirer) = plan_exchange_at_revision(
        initial_plan,
        &mut gate,
        0,
        PlanExchangeConfig {
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        },
    )
    .expect("exchange");
    let watermark = publisher.watermark_reader();
    let feed_block = |sources: &mut host_core::SourceControlSet, block: usize| {
        let range = block * quantum..(block + 1) * quantum;
        sources
            .submit(
                SOURCE.as_bytes(),
                SourceSubmission {
                    generation: 1,
                    start_frame: range.start as u64,
                    sample_rate_hz: 48_000,
                    planes: &[&planes[0][range.clone()], &planes[1][range]],
                    frames: quantum as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
    };
    assert_eq!(
        host.report.sample_rate_hz, 48_000,
        "the fixture's rate, which the feed names"
    );

    // The plan's latency delays the first audible block; the transaction lands after it, so a
    // changed strip changes the output. Blocks before `at` render the initial session.
    let at = host.report.latency_samples as usize / quantum + 2;
    assert!(at + 3 < 60, "the feed covers the blocks below");
    for block in 0..at {
        feed_block(&mut sources, block);
        render(&mut owner, quantum, block);
    }

    // One transaction: mute A, unmute B. Written into the cells under one stamp, not published.
    {
        let stamp_revision = revision;
        let stamp = gate.stamp(stamp_revision).expect("a legal revision");
        let mute = |muted| TrackFaderRecord::Mute {
            lanes: BuiltinLaneSelector::Both,
            muted,
            smoothing_samples: 0,
        };
        handles.strip_controls[A].fader.write(stamp, mute(true));
        handles.strip_controls[B].fader.write(stamp, mute(false));
    }

    // The next block starts with `r` unpublished: neither change is applied, and the watermark is
    // below.
    feed_block(&mut sources, at);
    let before = render(&mut owner, quantum, at);
    assert_eq!(
        before,
        reference_block(&initial, &planes, at),
        "an unpublished revision's writes are not applied"
    );
    assert_ne!(
        before,
        reference_block(&target, &planes, at),
        "non-vacuous: the transaction changes the output"
    );
    assert!(watermark.read().expect("watermark").revision < revision);

    // Publishing `r` makes both changes the next block's, from its first sample.
    gate.publish(revision);
    feed_block(&mut sources, at + 1);
    let after = render(&mut owner, quantum, at + 1);
    assert_eq!(
        after,
        reference_block(&target, &planes, at + 1),
        "both changes apply in the block whose snapshot covers the revision"
    );
    let published = watermark.read().expect("watermark");
    assert_eq!(
        (published.revision, published.first_sample),
        (revision, ((at + 1) * quantum) as u64),
        "the watermark names the block that applied the revision"
    );
}
