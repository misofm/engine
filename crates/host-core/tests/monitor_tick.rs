//! #1603 gate 5: a monitor write is a publication that carries the committed revision and spends
//! none, allocates nothing on either path, and applies by the same rule as every live value
//! (D15-2).
//!
//! * **The next block.** With no candidate pending, a strip mute written as a monitor publication
//!   under revision `r` applies from the next block's first sample, and the watermark still reads
//!   `(r, first sample of r)`: a monitor write never advances it.
//! * **A pending candidate.** A candidate published at revision `r + 1` and a monitor write under
//!   `r + 1` on its gate apply nothing before adoption; the adopting block applies both, and the
//!   watermark reads `(r + 1, that block's first sample)`.
//! * **Allocation-free.** After warm-up, the control thread's counters read `allocations == 0 &&
//!   deallocations == 0` around `stamp_monitor`, the cell write and `publish_key`, and around every
//!   render call.
//!
//! Red mutation: the gate orders by revision instead of publication (a monitor write then never
//! applies), or the watermark reads the publication number (it then names a number no model
//! commit holds).

use core::num::NonZeroUsize;

use bench_support::alloc as bench_alloc;
use builtins::BuiltinLaneSelector;
use builtins_compiler::TrackFaderRecord;
use dsp_reference::randomized::Draw;
use engine::realtime::{
    PlanAdoption, PlanExchangeConfig, PlanarBufferMut, RealtimePlanOwner, RenderIo, RenderTime,
    SwapOutcome, plan_exchange_at_revision,
};
use host_core::{
    HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes, HostPrepareCaps,
    HostShapePolicy, LiveValuesSuperseded, PreparedHost, SourceControlSet, SourceSubmission,
    compile_host_session, prepare_host_runtime, prepare_host_runtime_with_live_lanes,
};
use session::{Console, SessionModel, canonical_session_json, parse_session_json};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE: &str = "fixture-source";
/// The strip the monitor write mutes.
const MUTED: usize = 2;
const BLOCKS: usize = 64;

fn caps() -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 4_096,
        maximum_source_channels: None,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_submixes: 100,
        maximum_vcas: 100,
        maximum_vca_reach_pairs: u64::MAX,
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
/// strip is stateless downstream of its fader and a mute is a step at its first sample. Strip
/// `MUTED` is muted when `muted` is set.
fn model(muted: bool) -> SessionModel {
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
    model.tracks[MUTED].fader.left_mute = muted;
    model.tracks[MUTED].fader.right_mute = muted;
    model
}

fn compile(model: &SessionModel) -> session::CompiledSession {
    let document = canonical_session_json(model).expect("canonical");
    compile_host_session(&document, &caps()).unwrap_or_else(|failure| {
        panic!("compile: {}", String::from_utf8_lossy(failure.as_bytes()))
    })
}

fn feed() -> [Vec<f32>; 2] {
    let mut draw = Draw::new(0x1559);
    let mut plane = || -> Vec<f32> {
        (0..128 * BLOCKS)
            .map(|_| {
                let sample = draw.noise(0.5);
                if sample == 0.0 { 0.25 } else { sample }
            })
            .collect()
    };
    [plane(), plane()]
}

fn submit(sources: &mut SourceControlSet, planes: &[Vec<f32>; 2], quantum: usize, block: usize) {
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
}

fn bits(samples: &[f32]) -> Vec<u32> {
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
        submit(&mut host.sources, planes, quantum, current);
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
        last = bits(&samples);
    }
    last
}

fn prepare_live(model: &SessionModel) -> (PreparedHost, HostLiveControlHandles) {
    prepare_host_runtime_with_live_lanes(
        &compile(model),
        &caps(),
        &HostLiveControlRequest::default(),
        HostLiveLanes::FADER_AND_MATRIX,
        &LiveValuesSuperseded::default(),
    )
    .unwrap_or_else(|failure| panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes())))
}

/// Render one block and return its bits, counting the render call's allocator events.
fn render(owner: &mut RealtimePlanOwner, quantum: usize, block: usize) -> Vec<u32> {
    let mut samples = vec![0.0_f32; quantum * 2];
    let output = PlanarBufferMut::try_new(&mut samples, 2, quantum, quantum).expect("planes");
    let mark = bench_alloc::current_thread_counters();
    let result = owner.render(
        RenderIo { output },
        RenderTime {
            absolute_sample: (block * quantum) as u64,
        },
    );
    let delta = bench_alloc::current_thread_delta_since(mark);
    result.expect("render");
    assert_eq!(
        (delta.allocations, delta.deallocations),
        (0, 0),
        "block {block}: render allocated or freed"
    );
    bits(&samples)
}

fn mute_record() -> TrackFaderRecord {
    TrackFaderRecord::Mute {
        lanes: BuiltinLaneSelector::Both,
        muted: true,
        smoothing_samples: 0,
    }
}

fn exchange_config() -> PlanExchangeConfig {
    PlanExchangeConfig {
        retirement_capacity: NonZeroUsize::new(1).expect("one"),
    }
}

/// Test value: a monitor write that waits for a revision it does not spend (applies late), or one
/// that advances the watermark, or one that allocates on either path.
#[test]
fn a_monitor_write_applies_in_the_next_block_and_the_watermark_stays_on_the_revision() {
    bench_alloc::assert_installed();
    let planes = feed();
    let initial = model(false);
    let target = model(true);
    let (mut host, mut handles) = prepare_live(&initial);
    let quantum = host.report.quantum_frames as usize;
    assert_eq!(host.report.sample_rate_hz, 48_000);
    let mut gate = host
        .plan
        .take_revision_gate()
        .expect("the plan's gate handle");
    let revision = 3;
    let mut sources = host.sources;
    let (publisher, mut owner, _retirer) =
        plan_exchange_at_revision(host.plan, &mut gate, revision, exchange_config())
            .expect("exchange");
    let watermark = publisher.watermark_reader();

    let at = host.report.latency_samples as usize / quantum + 2;
    assert!(at + 12 < BLOCKS, "the feed covers the blocks below");
    for block in 0..at {
        submit(&mut sources, &planes, quantum, block);
        render(&mut owner, quantum, block);
    }
    // A model commit of r + 1 publishes `(r + 1, 0)`; the next block names it.
    let model_revision = revision + 1;
    let stamp = gate.stamp(model_revision).expect("the next revision");
    let key = stamp.key();
    gate.publish_key(key);
    submit(&mut sources, &planes, quantum, at);
    let before = render(&mut owner, quantum, at);
    assert_eq!(before, reference_block(&initial, &planes, at));
    let level = watermark.read().expect("watermark");
    let first_sample = (at * quantum) as u64;
    assert_eq!(
        (level.revision, level.first_sample),
        (model_revision, first_sample)
    );

    // The monitor write: a publication under `r + 1`, spending no revision. The control thread's
    // allocator events around the stamp, the cell write and the publication read zero.
    let mark = bench_alloc::current_thread_counters();
    {
        let stamp = gate.stamp_monitor(model_revision).expect("a monitor stamp");
        let key = stamp.key();
        assert_eq!((key.revision(), key.publication()), (model_revision, 3));
        handles.strip_controls[MUTED]
            .fader
            .write(stamp, mute_record());
        gate.publish_key(key);
    }
    let delta = bench_alloc::current_thread_delta_since(mark);
    assert_eq!(
        (delta.allocations, delta.deallocations),
        (0, 0),
        "the monitor publication allocated or freed"
    );
    submit(&mut sources, &planes, quantum, at + 1);
    let after = render(&mut owner, quantum, at + 1);
    assert_eq!(
        after,
        reference_block(&target, &planes, at + 1),
        "the mute applies from the next block's first sample"
    );
    assert_ne!(
        after,
        reference_block(&initial, &planes, at + 1),
        "non-vacuous: the mute changes the output"
    );
    let level = watermark.read().expect("watermark");
    assert_eq!(
        (level.revision, level.first_sample),
        (model_revision, first_sample),
        "a monitor write never advances the watermark"
    );

    // A second monitor write under the same revision applies the same way.
    {
        let stamp = gate
            .stamp_monitor(model_revision)
            .expect("a second monitor stamp");
        let key = stamp.key();
        assert_eq!((key.revision(), key.publication()), (model_revision, 4));
        handles.strip_controls[MUTED].fader.write(
            stamp,
            TrackFaderRecord::Mute {
                lanes: BuiltinLaneSelector::Both,
                muted: false,
                smoothing_samples: 0,
            },
        );
        gate.publish_key(key);
    }
    submit(&mut sources, &planes, quantum, at + 2);
    assert_eq!(
        render(&mut owner, quantum, at + 2),
        reference_block(&initial, &planes, at + 2),
        "the second monitor write applies in the next block"
    );
    assert_eq!(
        watermark.read().expect("watermark").revision,
        model_revision
    );
}

/// Test value: a monitor write on a pending candidate's gate that applies on the running plan,
/// that is lost at adoption, or whose adopting block names a lower revision.
#[test]
fn a_monitor_write_on_a_pending_candidate_applies_at_adoption_with_its_revision() {
    bench_alloc::assert_installed();
    let planes = feed();
    let initial = model(false);
    let target = model(true);
    let (mut host, _handles) = prepare_live(&initial);
    let quantum = host.report.quantum_frames as usize;
    let mut gate = host
        .plan
        .take_revision_gate()
        .expect("the plan's gate handle");
    let revision = 3;
    let mut sources = host.sources;
    let (mut publisher, mut owner, _retirer) =
        plan_exchange_at_revision(host.plan, &mut gate, revision, exchange_config())
            .expect("exchange");
    let watermark = publisher.watermark_reader();

    // The candidate carries the model commit r + 1 and is published for a later block.
    let (mut candidate, mut candidate_handles) = prepare_live(&initial);
    let mut candidate_gate = candidate
        .plan
        .take_revision_gate()
        .expect("the candidate's gate handle");
    let model_revision = revision + 1;
    let stamp = candidate_gate.stamp(model_revision).expect("a revision");
    let key = stamp.key();
    candidate_gate.publish_key(key);
    let at = host.report.latency_samples as usize / quantum + 2;
    let adopt = at + 2;
    assert!(adopt + 4 < BLOCKS, "the feed covers the blocks below");
    publisher
        .reserve_replacement(
            candidate.plan,
            PlanAdoption::NoEarlierThan((adopt * quantum) as u64),
        )
        .expect("room")
        .commit();

    for block in 0..=at {
        submit(&mut sources, &planes, quantum, block);
        let out = render(&mut owner, quantum, block);
        if block == at {
            assert_eq!(out, reference_block(&initial, &planes, at));
        }
    }
    assert_eq!(watermark.read().expect("watermark").revision, revision);

    // The monitor write goes to the newest plan's gate: the candidate's, `(r + 1, 1)`.
    let stamp = candidate_gate
        .stamp_monitor(model_revision)
        .expect("a monitor stamp on the candidate");
    let key = stamp.key();
    assert_eq!((key.revision(), key.publication()), (model_revision, 2));
    candidate_handles.strip_controls[MUTED]
        .fader
        .write(stamp, mute_record());
    candidate_gate.publish_key(key);

    // The running plan renders unchanged and the watermark stays until the adopting block.
    submit(&mut sources, &planes, quantum, at + 1);
    assert_eq!(
        render(&mut owner, quantum, at + 1),
        reference_block(&initial, &planes, at + 1),
        "nothing applies before adoption"
    );
    assert_eq!(watermark.read().expect("watermark").revision, revision);

    // The adopting block applies the model commit and the monitor write together.
    submit(&mut sources, &planes, quantum, adopt);
    let mut candidate_sources = candidate.sources;
    for block in 0..=adopt {
        submit(&mut candidate_sources, &planes, quantum, block);
    }
    let mut samples = vec![0.0_f32; quantum * 2];
    let output = PlanarBufferMut::try_new(&mut samples, 2, quantum, quantum).expect("planes");
    let report = owner
        .render(
            RenderIo { output },
            RenderTime {
                absolute_sample: (adopt * quantum) as u64,
            },
        )
        .expect("render");
    assert_eq!(report.swap, SwapOutcome::Applied);
    assert_eq!(
        (
            report.render.live_snapshot.revision(),
            report.render.live_snapshot.publication()
        ),
        (model_revision, 2)
    );
    let level = watermark.read().expect("watermark");
    assert_eq!(
        (level.revision, level.first_sample),
        (model_revision, (adopt * quantum) as u64)
    );
    // The candidate is an independent host: its sources read in order from its first block, so
    // the adopting block renders the candidate's block 0. It is muted from that first sample.
    let got = bits(&samples);
    assert_eq!(
        got,
        reference_block(&target, &planes, 0),
        "the adopting block applies the model commit and the monitor write together"
    );
    assert_ne!(
        got,
        reference_block(&initial, &planes, 0),
        "non-vacuous: the monitor mute changes the output"
    );
}
