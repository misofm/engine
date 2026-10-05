//! The swapped-run harness of *Swap a rebuilt plan without an audio gap* (#1269): render the old
//! plan, apply a structural edit by preparing a successor, swap it in through the engine's plan
//! exchange, render on; and render the post-edit session fresh from sample zero as the reference.
//! The two block streams are compared bit for bit. Slices 7-15 add their cases on top of it.

use core::num::NonZeroUsize;

use bench_support::alloc as bench_alloc;
use engine::realtime::{
    CarryOutcome, PlanExchangeConfig, PlanarBufferMut, RealtimePlanOwner, RenderIo, SwapOutcome,
    audit, plan_exchange,
};
use graph_compiler::Backend;
use host_core::{
    CompiledSession, HostLiveControlRequest, HostPrepareCaps, HostShapePolicy, PlanStateInventory,
    PrepareDiagnostics, PreparedHost, SourceControlSet, SourceSubmission, SuccessorBase,
    compile_host_model, test_only_prepare_host_runtime_with_live_controls_on,
    test_only_prepare_host_runtime_with_live_controls_successor_on,
};
use session::SessionModel;

/// Frames per block: the fixture's quantum.
pub(crate) const QUANTUM: usize = 128;
/// Blocks in every run.
pub(crate) const BLOCKS: usize = 12;
/// The first block the successor renders.
pub(crate) const SWAP_BLOCK: usize = 6;

/// Generous host caps; a test narrows one row when it needs to.
pub(crate) fn caps() -> HostPrepareCaps {
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

/// One session: the committed model and its compilation. The committed model is the compiled
/// session's normalized model, as the C ABI commits it, so a successor's join sees what production
/// hands it.
pub(crate) struct Session {
    pub(crate) model: SessionModel,
    pub(crate) compiled: CompiledSession,
}

impl Session {
    pub(crate) fn compile(model: SessionModel) -> Self {
        let compile_caps = caps()
            .compile_caps(model.sources.len())
            .unwrap_or_else(|_| panic!("compile caps"));
        let compiled = compile_host_model(&model, compile_caps)
            .unwrap_or_else(|failure| panic!("compile: {failure:?}"));
        Self {
            model: compiled.normalized_model().clone(),
            compiled,
        }
    }

    /// Prepared fresh, as a plan with no predecessor.
    pub(crate) fn prepare(&self, backend: Backend) -> PreparedHost {
        test_only_prepare_host_runtime_with_live_controls_on(
            &self.compiled,
            &caps(),
            &HostLiveControlRequest::default(),
            backend,
        )
        .unwrap_or_else(|failure| panic!("prepare: {failure:?}"))
        .0
    }

    /// Prepared as the successor of the plan `inventory` describes, whose committed model is
    /// `committed`.
    pub(crate) fn prepare_successor(
        &self,
        inventory: &PlanStateInventory,
        committed: &SessionModel,
        caps: &HostPrepareCaps,
        backend: Backend,
    ) -> Result<PreparedHost, PrepareDiagnostics> {
        test_only_prepare_host_runtime_with_live_controls_successor_on(
            &self.compiled,
            caps,
            &HostLiveControlRequest::default(),
            SuccessorBase {
                inventory,
                committed,
            },
            backend,
        )
        .map(|(prepared, _)| prepared)
    }
}

/// One source's PCM for the whole run: two planes, no exact-zero sample, distinct per `seed`.
pub(crate) struct Feed {
    pub(crate) id: &'static str,
    pub(crate) planes: [Vec<f32>; 2],
}

impl Feed {
    pub(crate) fn new(id: &'static str, seed: u32) -> Self {
        let plane = |channel: u32| {
            (0..(QUANTUM * BLOCKS) as u32)
                .map(|frame| {
                    let step = frame
                        .wrapping_mul(7_919)
                        .wrapping_add(seed.wrapping_mul(104_729))
                        .wrapping_add(channel * 31)
                        % 1_999;
                    // In [-0.49975, 0.49975] in steps of 1/2000, never 0: `step` is an integer.
                    (step as f32 - 999.5) / 2_000.0
                })
                .collect::<Vec<f32>>()
        };
        let planes = [plane(0), plane(1)];
        assert!(planes.iter().flatten().all(|sample| *sample != 0.0));
        Self { id, planes }
    }

    /// Submit block `block` through `sources` (generation 1, the ring's region from frame 0).
    pub(crate) fn submit(&self, sources: &mut SourceControlSet, rate: u32, block: usize) {
        let range = block * QUANTUM..(block + 1) * QUANTUM;
        sources
            .submit(
                self.id.as_bytes(),
                SourceSubmission {
                    generation: 1,
                    start_frame: range.start as u64,
                    sample_rate_hz: rate,
                    planes: &[&self.planes[0][range.clone()], &self.planes[1][range]],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .unwrap_or_else(|error| panic!("{} block {block}: {error:?}", self.id));
    }
}

/// How the swapped run builds its successor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Successor {
    /// [`Session::prepare_successor`] and `adopt_persisting`: the path under test.
    Carry,
    /// The successor prepared fresh, with no base: today's C ABI rebuild. Its rings are new and
    /// start at frame 0, so the playing frames cannot be fed to them without a seek; nothing is
    /// fed and the swap block is the gap the oracle must see.
    Fresh,
}

/// The rendered output of one block, as bits, and what the block reported.
pub(crate) struct Block {
    pub(crate) bits: Vec<u32>,
    pub(crate) swap: SwapOutcome,
    pub(crate) carry: CarryOutcome,
    /// `(allocations, reallocations, deallocations)` on the render thread during the render, and
    /// the render audit's `(allocations, deallocations)`.
    pub(crate) allocator: (u64, u64, u64),
    pub(crate) audit: (u64, u64),
}

fn render(owner: &mut RealtimePlanOwner) -> Block {
    let mut output = [f32::NAN; QUANTUM * 2];
    let sample = owner.next_absolute_sample();
    audit::reset();
    let mark = bench_alloc::current_thread_counters();
    let report = owner
        .render_contiguous(
            RenderIo {
                output: PlanarBufferMut::try_new(&mut output, 2, QUANTUM, QUANTUM).expect("output"),
            },
            sample,
        )
        .unwrap_or_else(|error| panic!("render: {error:?}"));
    let delta = bench_alloc::current_thread_delta_since(mark);
    let snapshot = audit::snapshot();
    Block {
        bits: output.iter().map(|sample| sample.to_bits()).collect(),
        swap: report.swap,
        carry: report.carry,
        allocator: (delta.allocations, delta.reallocations, delta.deallocations),
        audit: (snapshot.allocations, snapshot.deallocations),
    }
}

fn exchange(plan: engine::realtime::PreparedRenderPlan) -> ExchangeParts {
    plan_exchange(
        plan,
        PlanExchangeConfig {
            retirement_capacity: NonZeroUsize::MIN,
        },
    )
    .unwrap_or_else(|_| panic!("plan exchange"))
}

type ExchangeParts = (
    engine::realtime::PlanPublisher,
    RealtimePlanOwner,
    engine::realtime::PlanRetirer,
);

/// What the swapped run hands back besides its blocks, for a test's own checks after the swap.
pub(crate) struct SwappedRun {
    pub(crate) blocks: Vec<Block>,
    /// The successor's report.
    pub(crate) successor: host_core::HostPrepareReport,
    /// The producers each set held after the hand-over: the predecessor's, then the successor's.
    pub(crate) predecessor_sources: SourceControlSet,
    pub(crate) successor_sources: SourceControlSet,
    /// What `adopt_persisting` returned (zero for [`Successor::Fresh`]).
    pub(crate) adopted: usize,
}

/// Render `a` for [`SWAP_BLOCK`] blocks, prepare `b` as its successor (committed model: `a`'s),
/// swap it in, and render to [`BLOCKS`]. `before` feeds `a`, `after` feeds `b`, each one block
/// ahead of the render.
pub(crate) fn swapped_run(
    a: &Session,
    b: &Session,
    before: &[&Feed],
    after: &[&Feed],
    backend: Backend,
    successor: Successor,
) -> SwappedRun {
    let predecessor = a.prepare(backend);
    let rate = predecessor.report.sample_rate_hz;
    let PreparedHost {
        plan,
        sources: mut a_sources,
        inventory,
        ..
    } = predecessor;
    let (mut publisher, mut owner, _retirer) = exchange(plan);
    let mut blocks = Vec::with_capacity(BLOCKS);
    for feed in before {
        feed.submit(&mut a_sources, rate, 0);
    }
    for block in 0..SWAP_BLOCK {
        for feed in before {
            feed.submit(&mut a_sources, rate, block + 1);
        }
        blocks.push(render(&mut owner));
    }
    let (b_prepared, adopted) = match successor {
        Successor::Carry => {
            let mut prepared = b
                .prepare_successor(&inventory, &a.model, &caps(), backend)
                .unwrap_or_else(|failure| panic!("successor: {failure:?}"));
            let adopted = prepared.sources.adopt_persisting(&mut a_sources);
            (prepared, adopted)
        }
        Successor::Fresh => (b.prepare(backend), 0),
    };
    let PreparedHost {
        plan,
        sources: mut b_sources,
        report,
        ..
    } = b_prepared;
    publisher
        .reserve_replacement(plan)
        .unwrap_or_else(|_| panic!("reserve the successor"))
        .commit();
    for block in SWAP_BLOCK..BLOCKS {
        if successor == Successor::Carry && block + 1 < BLOCKS {
            for feed in after {
                feed.submit(&mut b_sources, rate, block + 1);
            }
        }
        blocks.push(render(&mut owner));
    }
    SwappedRun {
        blocks,
        successor: report,
        predecessor_sources: a_sources,
        successor_sources: b_sources,
        adopted,
    }
}

/// `session` prepared fresh and fed `feeds` from frame 0: the reference stream.
pub(crate) fn reference_run(session: &Session, feeds: &[&Feed], backend: Backend) -> Vec<Block> {
    let prepared = session.prepare(backend);
    let rate = prepared.report.sample_rate_hz;
    let PreparedHost {
        plan, mut sources, ..
    } = prepared;
    let (_publisher, mut owner, _retirer) = exchange(plan);
    for feed in feeds {
        feed.submit(&mut sources, rate, 0);
    }
    (0..BLOCKS)
        .map(|block| {
            if block + 1 < BLOCKS {
                for feed in feeds {
                    feed.submit(&mut sources, rate, block + 1);
                }
            }
            render(&mut owner)
        })
        .collect()
}

/// The first block whose output bits differ, if any.
pub(crate) fn first_difference(left: &[Block], right: &[Block]) -> Option<usize> {
    assert_eq!(left.len(), right.len());
    left.iter()
        .zip(right)
        .position(|(left, right)| left.bits != right.bits)
}
