//! Issue #1400 D4: render allocates nothing for a plan that carries an attachment.
//!
//! `engine` forbids `unsafe` and so cannot install an allocator; this crate has the audited one.
//! The render scope also aborts on any allocation (the default mode), so a stray allocation fails
//! this test twice over. The attachment's calls and its drop thread are held in
//! `crates/engine/tests/host_attachment.rs`.

#![allow(missing_docs)]

use std::num::NonZeroUsize;

use bench_support::alloc as bench_alloc;
use engine::realtime::{
    HostAttachment, PlanExchangeConfig, PlanarBufferMut, PrepareRenderPlan, PreparedRenderPlan,
    RenderEnvelope, RenderIo, RenderTime, SwapOutcome, plan_exchange,
};
use engine::{QuantumFrames, SampleRateHz};

/// Holds a heap block, so dropping it on the render path would be a deallocation.
struct Attachment(#[allow(dead_code)] Box<[u8; 256]>);

impl HostAttachment for Attachment {
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}

fn plan(id: u64) -> PreparedRenderPlan {
    let mut plan = PreparedRenderPlan::prepare(PrepareRenderPlan {
        plan_id: id,
        envelope: RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(2),
            output_channels: NonZeroUsize::new(1).expect("one"),
        },
        scratch: &[],
    })
    .expect("plan");
    plan.set_host_attachment(Box::new(Attachment(Box::new([0; 256]))));
    plan
}

/// Test value: red if render, or the swap block that displaces a plan, allocates, frees or drops
/// for the attachment: dropping a displaced plan's attachment at the swap would be a
/// deallocation inside the render scope, which no existing test sees because none of them carries
/// an attachment.
#[test]
fn a_plan_with_an_attachment_renders_and_swaps_with_zero_allocator_calls() {
    let (mut publisher, mut owner, mut retirer) = plan_exchange(
        plan(1),
        PlanExchangeConfig {
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        },
    )
    .expect("exchange");
    publisher
        .publish(plan(2), 0)
        .unwrap_or_else(|_| panic!("publish"));
    bench_alloc::assert_installed();

    let mut output = [1.0_f32; 2];
    let mut swaps = Vec::with_capacity(4);
    let mark = bench_alloc::current_thread_counters();
    for block in 0..4_u64 {
        let io = RenderIo {
            output: PlanarBufferMut::try_new(&mut output, 1, 2, 2).expect("output"),
        };
        let report = owner
            .render(
                io,
                RenderTime {
                    absolute_sample: block * 2,
                },
            )
            .expect("render");
        swaps.push(report.swap);
    }
    let moved = bench_alloc::current_thread_delta_since(mark);
    assert_eq!(swaps[0], SwapOutcome::Applied);
    assert_eq!((moved.allocations, moved.deallocations), (0, 0));
    // The displaced plan, attachment and all, drops here, off the render path.
    drop(retirer.try_reclaim().unwrap_or_else(|_| panic!("reclaim")));
}
