//! Fixed 10,000-callback audit of the production native dependency-wave graph.

#![allow(unsafe_code)]

use core::num::NonZeroUsize;
use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::mpsc,
};

use miso_engine_builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use miso_engine_core::realtime::{
    PlanExchangeConfig, PlanarBufferMut, RenderIo, RenderTime, SwapOutcome,
    audit::{self, AuditSnapshot, ForbiddenOperation, record_allocator_violation},
    plan_exchange,
};
use miso_engine_effect_compiler::EffectPreparedSession;
use miso_engine_graph::{
    GraphBindingBlock, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings, GraphRuntimeProcessor,
    NativeGraphBindConfigV1, NativeGraphRenderModeV1, NativeSchedulerConfigV1,
    SchedulerSelectionV1, TrackStage,
};
use miso_engine_graph_compiler::{GraphBuiltinsCompileRequest, GraphCompiler};
use miso_engine_session::{
    CompileCaps, RouteSource, SendTap, StableId, compile_session, parse_session_toml,
};

const CALLBACKS: u64 = 10_000;
const QUANTUM: usize = 128;

struct AuditedAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: AuditedAllocator = AuditedAllocator;

// SAFETY: every operation forwards the allocator's unchanged pointer/layout contract to System.
// An allocation or free on any armed render worker aborts instead of unwinding through GlobalAlloc.
unsafe impl GlobalAlloc for AuditedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: the allocator-provided layout is forwarded unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: the allocator-provided layout is forwarded unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        if record_allocator_violation(ForbiddenOperation::Deallocation) {
            std::process::abort();
        }
        // SAFETY: the original pointer/layout pair is forwarded unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: the original allocation contract and requested size are forwarded unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

struct Source {
    left: f32,
    right: f32,
}

impl GraphRuntimeProcessor for Source {
    fn process(
        &mut self,
        block: GraphBindingBlock<'_>,
    ) -> Result<(), miso_engine_core::realtime::RenderError> {
        block.left.fill(self.left);
        block.right.fill(self.right);
        Ok(())
    }
}

struct Identity;

impl GraphRuntimeProcessor for Identity {
    fn process(
        &mut self,
        _block: GraphBindingBlock<'_>,
    ) -> Result<(), miso_engine_core::realtime::RenderError> {
        Ok(())
    }
}

fn main() {
    assert_eq!(std::env::args_os().count(), 1, "audit accepts no arguments");
    let initial = prepared_graph(9_001);
    let replacement = prepared_graph(9_002);
    let (mut publisher, mut owner, retirer) = plan_exchange(
        initial,
        PlanExchangeConfig {
            publication_capacity: NonZeroUsize::new(1).expect("one"),
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        },
    )
    .expect("plan exchange");
    publisher
        .publish(replacement)
        .unwrap_or_else(|_| panic!("replacement publication"));

    let mut output = vec![0.0_f32; QUANTUM * 2];
    let output_address = output.as_ptr() as usize;
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    audit::warm_up();
    audit::reset();
    for block in 0..CALLBACKS {
        let report = owner
            .render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut output, 2, QUANTUM, QUANTUM)
                        .expect("fixed output"),
                },
                RenderTime {
                    absolute_sample: block * QUANTUM as u64,
                },
            )
            .expect("native graph render");
        assert_eq!(report.render.plan_id, 9_002);
        assert_eq!(
            report.swap,
            if block == 0 {
                SwapOutcome::Applied
            } else {
                SwapOutcome::None
            }
        );
        for sample in &output {
            hash = (hash ^ u64::from(sample.to_bits())).wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    assert_eq!(output.as_ptr() as usize, output_address);
    let coordinator = audit::snapshot();
    assert_eq!(coordinator.total(), 0);
    let mut workers = [AuditSnapshot::default(); 3];
    assert_eq!(owner.copy_worker_audit_snapshots(&mut workers), 3);
    assert!(workers.iter().all(|snapshot| snapshot.total() == 0));

    drop(publisher);
    let (sender, receiver) = mpsc::sync_channel(0);
    let retirement = std::thread::spawn(move || {
        let mut retirer = retirer;
        let retired = retirer.try_reclaim().expect("one displaced plan");
        drop(retired);
        drop(owner);
        sender
            .send(std::thread::current().id())
            .expect("retirement result");
    });
    let retirement_thread_id = receiver.recv().expect("retirement thread ID");
    assert_eq!(retirement.join().expect("retirement join"), ());
    println!(
        concat!(
            "{{\"schema_version\":1,\"kind\":\"native_scheduler_realtime_audit\",",
            "\"callbacks\":{},\"quantum_frames\":{},\"render_lanes\":4,",
            "\"worker_count\":3,\"plan_swaps\":1,\"retired_on_thread\":\"{:?}\",",
            "\"output_address\":{},\"output_hash\":{},",
            "\"coordinator_forbidden_total\":{},\"worker_forbidden_totals\":[{},{},{}]}}"
        ),
        CALLBACKS,
        QUANTUM,
        retirement_thread_id,
        output_address,
        hash,
        coordinator.total(),
        workers[0].total(),
        workers[1].total(),
        workers[2].total(),
    );
}

fn prepared_graph(plan_id: u64) -> miso_engine_core::realtime::PreparedRenderPlan {
    let mut model = parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
        .expect("canonical session");
    let base_track = model.tracks[0].clone();
    let base_route = model.routes[0].clone();
    model.automation.clear();
    model.tracks = (0..8)
        .map(|index| {
            let mut track = base_track.clone();
            track.id = StableId::parse(&format!("sched{index}")).expect("track ID");
            track.simd1.effects.clear();
            track.dynamic.effects.clear();
            track.simd2.effects.clear();
            track
        })
        .collect();
    model.routes = model
        .tracks
        .iter()
        .enumerate()
        .map(|(index, track)| {
            let mut route = base_route.clone();
            route.id = StableId::parse(&format!("scheduler-route-{index}")).expect("route ID");
            route.source = RouteSource::Track {
                track_id: track.id.clone(),
                tap: SendTap::PostMatrix,
            };
            route
        })
        .collect();
    let session = compile_session(
        &model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("compiled audit session");
    let builtins = prepare_session_builtins(
        &session,
        &[],
        BuiltinCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_total_retained_payload_bytes: u64::MAX,
            maximum_total_meter_items: u64::MAX,
            maximum_total_meter_bytes: u64::MAX,
            maximum_single_allocation_bytes: u64::MAX,
            maximum_meter_streams: u64::MAX,
            maximum_period_frames: u32::MAX,
            maximum_peak_hold_frames: u32::MAX,
            maximum_smoothing_samples: u32::MAX,
        },
    )
    .expect("prepared audit builtins");
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        plan_id,
        effects: EffectPreparedSession {
            session,
            entries: Vec::new(),
        },
        builtins,
        caps: miso_engine_graph::GraphCompileCaps {
            maximum_nodes: 10_000,
            maximum_edges: 10_000,
            maximum_schedule_items: 10_000,
            maximum_dependency_levels: 10_000,
            maximum_audio_buffer_samples: 10_000_000,
            maximum_delay_samples_per_edge: 1_000_000,
            maximum_total_delay_samples: 10_000_000,
            maximum_graph_bytes: 100_000_000,
            maximum_plan_bytes: 200_000_000,
            maximum_single_allocation_bytes: 100_000_000,
            maximum_finite_tail_samples: 10_000_000,
        },
    })
    .unwrap_or_else(|_| panic!("compiled audit graph"));
    assert!(artifact.prepared_builtin_bank_count() >= 1);
    let envelope = artifact.envelope();
    assert_eq!(envelope.quantum.0 as usize, QUANTUM);
    let nodes = artifact
        .external_binding_nodes()
        .cloned()
        .enumerate()
        .map(|(index, node)| {
            let processor: Box<dyn GraphRuntimeProcessor> = match node {
                GraphNodeId::TrackStage {
                    stage: TrackStage::Input,
                    ..
                } => Box::new(Source {
                    left: index as f32 * 0.01 + 0.1,
                    right: index as f32 * -0.02 - 0.2,
                }),
                _ => Box::new(Identity),
            };
            GraphNodeBinding::new(node, processor)
        })
        .collect();
    let bound = artifact
        .into_bound_native(
            GraphRuntimeBindings {
                envelope,
                nodes,
                observers: Vec::new(),
            },
            NativeGraphBindConfigV1 {
                render_mode: NativeGraphRenderModeV1::DependencyWaves,
                scheduler: NativeSchedulerConfigV1::new(
                    NonZeroUsize::new(4).expect("four lanes"),
                    true,
                ),
                maximum_retained_bytes: 1 << 29,
            },
        )
        .unwrap_or_else(|failure| panic!("native audit bind: {}", failure.code));
    assert_eq!(
        bound.prepared.metadata.selection,
        SchedulerSelectionV1::Parallel
    );
    assert_eq!(bound.prepared.metadata.resources.scheduler.worker_count, 3);
    bound.prepared.into_plan()
}
